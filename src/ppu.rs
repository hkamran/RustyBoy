use crate::mmu::Mmu;

pub const VRAM_SIZE: usize = 0x4000;
pub const VOAM_SIZE: usize = 0xA0;
pub const SCREEN_W: usize = 160;
pub const SCREEN_H: usize = 144;

#[derive(PartialEq, Copy, Clone)]
enum PriorityType {
    Normal,
}

#[allow(dead_code)]
pub struct Ppu {
    // 0xFF40 (http://bgb.bircd.org/pandocs.htm#videodisplay)
    lcd_display_enable: bool,
    window_tile_map_select: u16,
    window_display_enable: bool,
    tilemap_base_select: u16,
    bg_tile_map: u16,
    sprite_size: u32,
    sprite_enable: bool,
    bg_display: bool,

    // 0xFF41 (http://bgb.bircd.org/pandocs.htm#lcdstatusregister)
    lyc_interrupt_enable: bool,
    mode_2_interrupt: bool,
    mode_1_interrupt: bool,
    mode_0_interrupt: bool,

    // http://bgb.bircd.org/pandocs.htm#lcdpositionandscrolling
    scroll_y_coord: u8,
    scroll_x_coord: u8,
    window_y_coord: u8,
    window_x_coord: u8,
    lcd_y_coordinate: u8,
    lcd_ly_compare: u8,

    // http://bgb.bircd.org/pandocs.htm#lcdmonochromepalettes
    gb_bg_palette: u8,
    gb_obj_palette_0: u8,
    gb_obj_palette_1: u8,

    // http://bgb.bircd.org/pandocs.htm#lcdcolorpalettescgbonly
    cbg_bg_palette_index: u8,
    cbg_bg_palette_increment: bool,
    cbg_bg_palette: [[[u8; 3]; 4]; 8],

    cbg_obj_index: u8,
    cbg_obj_increment: bool,
    cbg_obj: [[[u8; 3]; 4]; 8],

    vram_bank: usize,
    vram: [u8; VRAM_SIZE],
    voam: [u8; VOAM_SIZE],

    bg_priority: [PriorityType; SCREEN_W],

    frame: [u8; SCREEN_W * SCREEN_H * 3],
    interrupt: u8,

    h_blank: bool,
    v_blank: bool,

    mode: u8,
    cycles: u32,

    y: u8,
    x: u8,
}

#[allow(dead_code)]
impl Ppu {

    pub fn new() -> Self {
        return Ppu {
            lcd_display_enable: false,
            window_tile_map_select: 0x9C00,
            window_display_enable: false,
            tilemap_base_select: 0x8000,
            bg_tile_map: 0x9C00,
            sprite_size: 8,
            sprite_enable: false,
            bg_display: false,

            lyc_interrupt_enable: false,
            mode_2_interrupt: false,
            mode_1_interrupt: false,
            mode_0_interrupt: false,

            scroll_y_coord: 0,
            scroll_x_coord: 0,
            window_y_coord: 0,
            window_x_coord: 0,
            lcd_y_coordinate: 0,
            lcd_ly_compare: 0,

            gb_bg_palette: 0,
            gb_obj_palette_0: 0,
            gb_obj_palette_1: 0,

            cbg_bg_palette_index: 0,
            cbg_bg_palette_increment: false,
            cbg_bg_palette: [[[0u8; 3]; 4]; 8],

            cbg_obj_index: 0,
            cbg_obj_increment: false,
            cbg_obj: [[[0u8; 3]; 4]; 8],

            vram_bank: 0,
            vram: [0; VRAM_SIZE],
            voam: [0; VOAM_SIZE],

            bg_priority: [PriorityType::Normal; SCREEN_W],
            frame: [0; SCREEN_W * SCREEN_H * 3],
            interrupt: 0,

            h_blank: false,
            v_blank: false,

            cycles: 0,
            mode: 0,
            y: 0,
            x: 0,
        };
    }

    #[allow(unused)]
    pub fn tick(&mut self, bus: &mut Mmu) -> () {
        if !self.lcd_display_enable {
            return;
        }
        self.h_blank = false;
        self.cycles += 1;

        if self.cycles >= 456 {
            self.cycles = 0;
        }
    }

    pub fn read_byte(&mut self, address: u16) -> u8 {
        match address {
            0x8000 ..= 0x9FFF => self.vram[(self.vram_bank * 0x2000) | (address as usize & 0x1FFF)],
            0xFE00 ..= 0xFE9F => self.voam[address as usize - 0xFE00],
            0xFF40 => {
                (if self.lcd_display_enable { 0x80 } else { 0 }) |
                    (if self.window_tile_map_select == 0x9C00 { 0x40 } else { 0 }) |
                    (if self.window_display_enable { 0x20 } else { 0 }) |
                    (if self.tilemap_base_select == 0x8000 { 0x10 } else { 0 }) |
                    (if self.bg_tile_map == 0x9C00 { 0x08 } else { 0 }) |
                    (if self.sprite_size == 16 { 0x04 } else { 0 }) |
                    (if self.sprite_enable { 0x02 } else { 0 }) |
                    (if self.bg_display { 0x01 } else { 0 })
            },
            0xFF41 => {
                (if self.lyc_interrupt_enable { 0x40 } else { 0 }) |
                    (if self.mode_2_interrupt { 0x20 } else { 0 }) |
                    (if self.mode_1_interrupt { 0x10 } else { 0 }) |
                    (if self.mode_0_interrupt { 0x08 } else { 0 }) |
                    (if self.y == self.lcd_y_coordinate { 0x04 } else { 0 }) |
                    self.mode
            },
            0xFF42 => self.scroll_y_coord,
            0xFF43 => self.scroll_x_coord,
            0xFF44 => self.y,
            0xFF45 => self.lcd_ly_compare,
            0xFF46 => 0,
            0xFF47 => self.gb_bg_palette,
            0xFF48 => self.gb_obj_palette_0,
            0xFF49 => self.gb_obj_palette_1,
            0xFF4A => self.window_y_coord,
            0xFF4B => self.window_x_coord,
            0xFF4F => self.vram_bank as u8,
            0xFF68 => { self.cbg_bg_palette_index | (if self.cbg_bg_palette_increment { 0x80 } else { 0 }) },
            0xFF69 => {
                let palnum = (self.cbg_bg_palette_index >> 3) as usize;
                let colnum = ((self.cbg_bg_palette_index >> 1) & 0x3) as usize;
                if self.cbg_bg_palette_index & 0x01 == 0x00 {
                    self.cbg_bg_palette[palnum][colnum][0] | ((self.cbg_bg_palette[palnum][colnum][1] & 0x07) << 5)
                } else {
                    ((self.cbg_bg_palette[palnum][colnum][1] & 0x18) >> 3) | (self.cbg_bg_palette[palnum][colnum][2] << 2)
                }
            },
            0xFF6A => { self.cbg_obj_index | (if self.cbg_obj_increment { 0x80 } else { 0 }) },
            0xFF6B => {
                let palnum = (self.cbg_obj_index >> 3) as usize;
                let colnum = ((self.cbg_obj_index >> 1) & 0x3) as usize;
                if self.cbg_obj_index & 0x01 == 0x00 {
                    self.cbg_obj[palnum][colnum][0] | ((self.cbg_obj[palnum][colnum][1] & 0x07) << 5)
                } else {
                    ((self.cbg_obj[palnum][colnum][1] & 0x18) >> 3) | (self.cbg_obj[palnum][colnum][2] << 2)
                }
            },
            _ => panic!("invalid"),
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x8000 ..= 0x9FFF => self.vram[(self.vram_bank * 0x2000) | (address as usize & 0x1FFF)] = value,
            0xFE00 ..= 0xFE9F => self.voam[address as usize - 0xFE00] = value,
            0xFF40 => {
                self.lcd_display_enable = value & 0x80 == 0x80;
                self.window_tile_map_select = if value & 0x40 == 0x40 { 0x9C00 } else { 0x9800 };
                self.window_display_enable = value & 0x20 == 0x20;
                self.tilemap_base_select = if value & 0x10 == 0x10 { 0x8000 } else { 0x8800 };
                self.bg_tile_map = if value & 0x08 == 0x08 { 0x9C00 } else { 0x9800 };
                self.sprite_size = if value & 0x04 == 0x04 { 16 } else { 8 };
                self.sprite_enable = value & 0x02 == 0x02;
                self.bg_display = value & 0x01 == 0x01;
            },
            0xFF41 => {
                self.lyc_interrupt_enable = value & 0x40 == 0x40;
                self.mode_2_interrupt = value & 0x20 == 0x20;
                self.mode_1_interrupt = value & 0x10 == 0x10;
                self.mode_0_interrupt = value & 0x08 == 0x08;
            },
            0xFF42 => self.scroll_y_coord = value,
            0xFF43 => self.scroll_x_coord = value,
            0xFF44 => {},
            0xFF45 => self.lcd_ly_compare = value,
            0xFF46 => {},
            0xFF47 => { self.gb_bg_palette = value; },
            0xFF48 => { self.gb_obj_palette_0 = value; },
            0xFF49 => { self.gb_obj_palette_1 = value; },
            0xFF4A => self.window_y_coord = value,
            0xFF4B => self.window_x_coord = value,
            0xFF4F => self.vram_bank = (value & 0x01) as usize,
            0xFF68 => { self.cbg_bg_palette_index = value & 0x3F; self.cbg_bg_palette_increment = value & 0x80 == 0x80; },
            0xFF69 => {
                let pal_num = (self.cbg_bg_palette_index >> 3) as usize;
                let col_num = ((self.cbg_bg_palette_index >> 1) & 0x03) as usize;
                if self.cbg_bg_palette_index & 0x01 == 0x00 {
                    self.cbg_bg_palette[pal_num][col_num][0] = value & 0x1F;
                    self.cbg_bg_palette[pal_num][col_num][1] = (self.cbg_bg_palette[pal_num][col_num][1] & 0x18) | (value >> 5);
                } else {
                    self.cbg_bg_palette[pal_num][col_num][1] = (self.cbg_bg_palette[pal_num][col_num][1] & 0x07) | ((value & 0x3) << 3);
                    self.cbg_bg_palette[pal_num][col_num][2] = (value >> 2) & 0x1F;
                }
                if self.cbg_bg_palette_increment { self.cbg_bg_palette_index = (self.cbg_bg_palette_index + 1) & 0x3F; };
            },
            0xFF6A => { self.cbg_obj_index = value & 0x3F; self.cbg_obj_increment = value & 0x80 == 0x80; },
            0xFF6B => {
                let pal_num = (self.cbg_obj_index >> 3) as usize;
                let col_num = ((self.cbg_obj_index >> 1) & 0x03) as usize;
                if self.cbg_obj_index & 0x01 == 0x00 {
                    self.cbg_obj[pal_num][col_num][0] = value & 0x1F;
                    self.cbg_obj[pal_num][col_num][1] = (self.cbg_obj[pal_num][col_num][1] & 0x18) | (value >> 5);
                } else {
                    self.cbg_obj[pal_num][col_num][1] = (self.cbg_obj[pal_num][col_num][1] & 0x07) | ((value & 0x3) << 3);
                    self.cbg_obj[pal_num][col_num][2] = (value >> 2) & 0x1F;
                }
                if self.cbg_obj_increment { self.cbg_obj_index = (self.cbg_obj_index + 1) & 0x3F; };
            },
            _ => panic!("invalid"),
        }
    }

}