use crate::mmu::Mmu;

pub const VRAM_SIZE: usize = 0x4000;
pub const VOAM_SIZE: usize = 0xA0;
pub const SCREEN_W: usize = 160;
pub const SCREEN_H: usize = 144;
pub const INTERRUPT_TIMER_MASK: u8 = 0x02;
pub const INTERRUPT_V_BLANK_MASK: u8 = 0x01;

#[derive(PartialEq, Copy, Clone)]
enum PriorityType {
    Normal,
}


// https://gbdev.io/pandocs/#ff41-stat-lcdc-status-r-w
#[derive(PartialEq, Copy, Clone)]
enum GpuMode {
    Read = 2,
    Transfer = 3,
    HBlank = 0,
    VBlank = 1,
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
    lyc: u8,

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

    // https://gbdev.io/pandocs/#ff0f-if-interrupt-flag-r-w
    interrupt: u8,

    h_blank: bool,
    v_blank: bool,

    mode: GpuMode,
    clock: u32,
    ly: u8,
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
            lyc: 0,

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

            clock: 0,
            mode: GpuMode::VBlank,
            ly: 0,
        };
    }

    #[allow(unused)]
    pub fn tick(&mut self, bus: &mut Mmu) -> () {
        if !self.lcd_display_enable {
            return;
        }

        self.clock += 1;

        // http://imrannazar.com/GameBoy-Emulation-in-JavaScript:-GPU-Timings
        // https://gbdev.io/pandocs/#pixel-fifo
        match self.mode {
            GpuMode::Read => {
                if self.clock >= 80 {
                    self.set_mode(GpuMode::Transfer);
                    self.clock = 0;
                }
            },
            GpuMode::Transfer => {
                if self.clock >= 172 {
                    self.set_mode(GpuMode::HBlank);
                    self.clock = 0;

                    // Render scanline here
                    self.render_scan_line();
                }
            },
            GpuMode::HBlank => {
                if self.clock >= 204 {
                    self.clock = 0;
                    self.ly += 1;

                    // Check interrupt here
                    self.update_interrupt_for_lyc();

                    if self.ly == 143 {
                        self.set_mode(GpuMode::VBlank);
                        self.interrupt = INTERRUPT_V_BLANK_MASK;

                        // print frame here
                        self.render_frame();
                    } else {
                        self.set_mode(GpuMode::Read);
                    }

                }
            },
            GpuMode::VBlank => {
                if self.clock >= 456 {
                    self.clock = 0;
                    self.ly += 1;

                    if self.ly > 153 {
                        self.ly = 0;
                        self.set_mode(GpuMode::Read);
                    }
                }
            },
            _ => {
                panic!("error");
            }
        };

    }

    fn render_frame(&mut self) {

    }

    fn render_scan_line(&mut self) {
        for x in 0 .. SCREEN_W {
            self.set_rgb_at(x, self.ly, 255, 255, 255);
            self.bg_priority[x] = PriorityType::Normal;
        }

        self.render_bg_line();
        self.render_sprite_line();
    }

    fn render_bg_line(&mut self) {

    }

    fn render_sprite_line(&mut self) {

    }

    fn set_rgb_at(&mut self, x: usize, y: u8, red: u8, green: u8, blue: u8) {
        let base = (y as usize) * SCREEN_W * 3 + x * 3;

        let r = red as u32;
        let g = green as u32;
        let b = blue as u32;

        self.frame[base + 0] = ((r * 13 + g * 2 + b) >> 1) as u8;
        self.frame[base + 1] = ((g * 3 + b) << 1) as u8;
        self.frame[base + 2] = ((r * 3 + g * 2 + b * 11) >> 1) as u8;
    }

    fn set_mode(&mut self, mode: GpuMode) {
        self.mode = mode;

        match self.mode {
            GpuMode::VBlank => {
                self.v_blank = true;
                self.h_blank = false;
            },
            GpuMode::HBlank => {
                self.h_blank = true;
            },
            GpuMode::Read => {
                self.h_blank = false;
                self.v_blank = false;
            },
            GpuMode::Transfer => {
                self.h_blank = false;
                self.v_blank = false;
            }
        }

        self.update_interrupt_for_mode();
    }

    fn update_interrupt_for_mode(&mut self) {
        if self.mode == GpuMode::Read && self.mode_2_interrupt {
            self.interrupt |= INTERRUPT_TIMER_MASK;
        }
        if self.mode == GpuMode::HBlank && self.mode_0_interrupt {
            self.interrupt |= INTERRUPT_TIMER_MASK;
        }
        if self.mode == GpuMode::VBlank && self.mode_1_interrupt {
            self.interrupt |= INTERRUPT_TIMER_MASK;
        }
    }

    fn update_interrupt_for_lyc(&mut self) {
        if self.lyc_interrupt_enable {
            if self.ly == self.lyc {
                self.interrupt |= INTERRUPT_TIMER_MASK;
            }
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
                    (if self.ly == self.lcd_y_coordinate { 0x04 } else { 0 }) |
                    self.mode as u8
            },
            0xFF42 => self.scroll_y_coord,
            0xFF43 => self.scroll_x_coord,
            0xFF44 => self.ly,
            0xFF45 => self.lyc,
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
            0xFF45 => self.lyc = value,
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