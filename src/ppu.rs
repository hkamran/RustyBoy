use crate::screen::Screen;
use crate::console::GameboyType;
use crate::logger::log;

pub const VRAM_SIZE: usize = 0x4000;
pub const VOAM_SIZE: usize = 0xA0;
pub const SCREEN_W: usize = 160;
pub const SCREEN_H: usize = 144;
pub const INTERRUPT_TIMER_MASK: u8 = 0x02;
pub const INTERRUPT_V_BLANK_MASK: u8 = 0x01;

#[derive(PartialEq, Copy, Clone)]
enum PaletteType {
    BACKGROUND,
    OBJECTS
}

pub struct TileData {
    tile_1: u8,
    tile_2: u8,
}

pub struct TileEntry {
    palette_number: usize,
    vram_bank: u8,
    x_flip: bool,
    y_flip: bool,
    has_priority: bool,
}

pub struct SpriteOam {
    y_cord: i32,
    x_cord: i32,
    tile_number: u16,
    x_flip: bool,
    y_flip: bool,
    has_priority: bool,
    palette_number: u8,
    vram_bank: u8,
    pal_palette_index: u8,
}

#[derive(PartialEq, Copy, Clone)]
enum PriorityType {
    None,
    BgColor0,
    BgPriority
}

// https://gbdev.io/pandocs/#ff41-stat-lcdc-status-r-w
#[derive(PartialEq, Copy, Clone)]
pub enum GpuMode {
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
    bg_tile_data_select: u16,
    bg_tile_map_select: u16,
    sprite_size: i32,
    sprite_enable: bool,
    bg_display_enable: bool,

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
    pal_bg_palette_data: u8,
    pal_obj_palette_0_data: u8,
    pal_obj_palette_1_data: u8,

    pal_bg_palette: [u8; 4],
    pal_obj_palette_0: [u8; 4],
    pal_obj_palette_1: [u8; 4],

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

    scanline_priority: [PriorityType; SCREEN_W],

    frame: [u8; SCREEN_W * SCREEN_H * 3],

    // https://gbdev.io/pandocs/#ff0f-if-interrupt-flag-r-w
    pub interrupt_flags: u8,
    obj_master_priority: bool,

    pub h_blank: bool,
    pub v_blank: bool,

    mode: GpuMode,
    clock: u32,
    ly: u8,
    wly: u8,
    model: GameboyType,

    screen: Screen
}

#[allow(dead_code)]
impl Ppu {

    pub fn new() -> Self {
        return Ppu {
            lcd_display_enable: true,
            window_tile_map_select: 0x9800,
            window_display_enable: false,
            bg_tile_data_select: 0x8000,
            bg_tile_map_select: 0x9800,
            sprite_size: 8,
            sprite_enable: false,
            bg_display_enable: true,

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

            pal_bg_palette_data: 0xFC,
            pal_obj_palette_0_data: 0xFF,
            pal_obj_palette_1_data: 0xFF,

            pal_bg_palette: [0; 4],
            pal_obj_palette_0: [0; 4],
            pal_obj_palette_1: [0; 4],

            cbg_bg_palette_index: 0,
            cbg_bg_palette_increment: false,
            cbg_bg_palette: [[[0u8; 3]; 4]; 8],

            cbg_obj_index: 0,
            cbg_obj_increment: false,
            cbg_obj: [[[0u8; 3]; 4]; 8],

            vram_bank: 0,
            vram: [0; VRAM_SIZE],
            voam: [0; VOAM_SIZE],

            scanline_priority: [PriorityType::None; SCREEN_W],
            frame: [0; SCREEN_W * SCREEN_H * 3],
            interrupt_flags: 0,
            obj_master_priority: false,

            h_blank: false,
            v_blank: false,

            clock: 0,
            mode: GpuMode::VBlank,
            ly: 0,
            wly: 0,
            model: GameboyType::CLASSIC,

            screen: Screen::new()
        };
    }

    pub fn reset(&mut self, model: GameboyType) {
        self.interrupt_flags = 0;
        self.h_blank = false;
        self.v_blank = false;
        self.clock = 0;
        self.mode = GpuMode::Read;
        self.model = model;
        self.ly = 0;
    }

    pub fn execute_ticks(&mut self, ticks: u32) -> () {
        for _i in 0 .. ticks {
            self.execute_tick();
        }
    }

    #[allow(unused)]
    pub fn execute_tick(&mut self) -> () {
        // TODO LOOK OVER THIS
        // if !self.lcd_display_enable {
        //     return;
        // }

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

                    if self.ly >= 143 {
                        self.set_mode(GpuMode::VBlank);
                        self.interrupt_flags |= INTERRUPT_V_BLANK_MASK;

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
                        self.wly = 0;
                        self.obj_master_priority = false;
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
        let mut buffer: Vec<u32> = vec![0; SCREEN_W * SCREEN_H];

        fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
            let (r, g, b) = (r as u32, g as u32, b as u32);
            (r << 16) | (g << 8) | b
        }

        let mut index = 0;
        for y in 0 .. SCREEN_H {
            for x in 0 .. SCREEN_W {
                let base = (y as usize) * SCREEN_W * 3 + x as usize * 3;
                let r = self.frame[base + 0];
                let g = self.frame[base + 1];
                let b = self.frame[base + 2];
                buffer[index] = from_u8_rgb(r, g, b);
                index += 1;
            }
        }

        self.screen.update(buffer);
    }

    fn render_scan_line(&mut self) {
        for x in 0 .. SCREEN_W {
            self.set_rgb_at(x, self.ly as usize, 255, 255, 255);
            self.scanline_priority[x] = PriorityType::None;
        }

        self.render_bg_line();
        self.render_sprite_line();
    }

    fn render_bg_line(&mut self) {
        let mut window_has_rendered = false;

        let draw_window = self.window_display_enable && self.model == GameboyType::COLOR;
        let draw_background = self.model == GameboyType::COLOR || self.bg_display_enable;

        if !draw_window && !draw_background { return; }

        let display_y = self.ly as usize;
        for display_x in 0 .. SCREEN_W {;
            let mut window_y = if (-(self.window_y_coord as i32) + (display_y as i32)) >= 0 { self.wly as i32 } else { -1 };
            let mut window_x = (-((self.window_x_coord as i32) - 7) + (display_x as i32));

            // Values in range from 0-255 may be used for X/Y each, the video controller automatically
            // wraps back to the upper (left) position in BG map when drawing exceeds the lower (right) border of the BG map area.
            let bg_y = (display_y as u8).wrapping_add(self.scroll_y_coord) as usize;
            let bg_x = (display_x as u8).wrapping_add(self.scroll_x_coord) as usize;

            let mut tile_x= 0;
            let mut tile_y= 0;

            let mut pixel_x= 0;
            let mut pixel_y= 0;

            let mut tile_map_base_address= 0;

            let is_window_in_range = window_y >= 0 && window_x >= 0;
            if draw_window && is_window_in_range {
                tile_map_base_address = self.window_tile_map_select;
                tile_x = (window_x / 8) as u16;
                tile_y = (window_y / 8) as u16;
                pixel_x = (window_x % 8) as u16;
                pixel_y = (window_y % 8) as u16;

                window_has_rendered = true;
            } else if draw_background {
                tile_map_base_address = self.bg_tile_map_select;
                tile_x = (bg_x / 8) as u16;
                tile_y = (bg_y / 8) as u16;
                pixel_x = (bg_x % 8) as u16;
                pixel_y = (bg_y % 8) as u16;
            } else {
                continue;
            }

            // It is organized as 32 rows of 32 bytes each.
            // Each byte contains a number of a tile to be displayed.

            let tile_map_index = (tile_y * 32) + tile_x;
            let tile_map_address = tile_map_base_address + tile_map_index;

            let attributes: TileEntry = self.get_bg_tile_attributes(tile_map_address);
            let tile: TileData = self.get_bg_tile_at_y(tile_map_address, attributes.y_flip, pixel_y as u16, attributes.vram_bank);

            let bit_mask = match attributes.x_flip {
                true => pixel_x,
                false => 7 - pixel_x,
            } as u32;

            let palette_index = if tile.tile_1 & (1 << bit_mask) != 0 { 1 } else { 0 }
                | if tile.tile_2 & (1 << bit_mask) != 0 { 2 } else { 0 };

            self.scanline_priority[display_x] =
                if palette_index == 0 { PriorityType::BgColor0 }
                else if attributes.has_priority { PriorityType::BgPriority }
                else { PriorityType::None };

            if self.model == GameboyType::COLOR {
                let r = self.cbg_bg_palette[attributes.palette_number][palette_index][0];
                let g = self.cbg_bg_palette[attributes.palette_number][palette_index][1];
                let b = self.cbg_bg_palette[attributes.palette_number][palette_index][2];

                self.set_rgb_at(display_x as usize, self.ly as usize, r, g, b);
            } else {
                let r = self.pal_bg_palette[palette_index];
                let g = self.pal_bg_palette[palette_index];
                let b = self.pal_bg_palette[palette_index];

                self.set_rgb_at(display_x as usize, self.ly as usize, r, g, b);
            }
        }

        // https://sudonull.com/post/136059-Writing-a-Gameboy-Emulator-Part-2
        // To display it, a hidden pointer to the current line of the “window” is used, which increments
        // after the output of the next line. If the “window” is disabled or hidden due to WX / WY coordinates,
        // then its line counter is not incremented. Thus, if the output of the "window" was disabled halfway,
        // then when the output is turned on, the output will continue from where it left off.
        // This is true for one screen update. At the end of the V-blank, the “window” line counter is reset to zero.

        if window_has_rendered {
            self.wly += 1;
        }
    }

    fn get_bg_tile_attributes(&mut self, tile_map_address: u16) -> TileEntry {
        if self.model == GameboyType::CLASSIC {
            return TileEntry{
                palette_number : 0,
                vram_bank: 0,
                x_flip: false,
                y_flip: false,
                has_priority: false,
            }
        }

        let tile_map = self.read_byte_from_vram(1, tile_map_address) as usize;

        // Bit 0-2  Background Palette number  (BGP0-7)
        // Bit 3    Tile VRAM Bank number      (0=Bank 0, 1=Bank 1)
        // Bit 4    Not used
        // Bit 5    Horizontal Flip            (0=Normal, 1=Mirror horizontally)
        // Bit 6    Vertical Flip              (0=Normal, 1=Mirror vertically)
        // Bit 7    BG-to-OAM Priority         (0=Use OAM priority bit, 1=BG Priority)

        let palette_number = tile_map & 0x07;
        let vram_bank = if tile_map & 0x8 > 0 {1} else {0};
        let x_flip = tile_map & 0x20 != 0;
        let y_flip = tile_map & 0x40 != 0;
        let has_priority = tile_map & 0x80 != 0;

        return TileEntry{
            palette_number,
            vram_bank,
            x_flip,
            y_flip,
            has_priority,
        }
    }

    fn get_bg_tile_at_y(&mut self, tile_map_address: u16, y_flip: bool, pixel_y: u16, bank: u8) -> TileData {
        // An area of VRAM known as Background Tile Map contains the tile id to be displayed.
        // Each byte in the memory region is a tile identification number of what needs to be drawn.
        // This identification number is used to lookup the tile data in video ram so we know how to draw it.
        // It is organized as 32 rows of 32 bytes each. Each byte contains a number of a tile to be displayed.
        // Tile patterns are taken from the Tile Data Table located either at $8000-8FFF or $8800-97FF.
        // http://www.codeslinger.co.uk/pages/projects/gameboy/graphics.html

        let tile_pattern_index = self.read_byte_from_vram(0, tile_map_address) as u16;

        // In the first case, patterns are numbered with unsigned numbers from 0 to 255 (i.e. pattern #0 lies at address $8000).
        // In the second case, patterns have signed numbers from -128 to 127 (i.e. pattern #0 lies at address $9000).

        // Each Tile occupies 16 bytes, where each 2 bytes represent a line:

        let tile_pattern_base_address =
            if self.bg_tile_data_select == 0x8000 {
                let offset = tile_pattern_index * 16;
                let address = self.bg_tile_data_select + offset;
                address
            }
            else {
                let offset = ((tile_pattern_index as i8) as i16 + 128) * 16;
                let address = (self.bg_tile_data_select as i16) + offset;
                address as u16
            };

        // A sprite is 8x8 and each line is made up of 2 bytes
        let tile_pattern_offset = match y_flip {
            false => (pixel_y * 2),
            true => (14 - (pixel_y * 2)),
        };

        let tile_pattern_address = tile_pattern_base_address + tile_pattern_offset;

        let tile_1 = self.read_byte_from_vram(bank, tile_pattern_address);
        let tile_2 = self.read_byte_from_vram(bank, tile_pattern_address + 1);

        return TileData{
            tile_1,
            tile_2
        }
    }

    fn render_sprite_line(&mut self) {
        if !self.sprite_enable {
            return;
        }

        let mut sprite_counter = 0;

        let display_y = self.ly as i32;
        let sprite_size = self.sprite_size as i32;

        // https://gbdev.io/pandocs/#fifo-pixel-fetcher
        // http://imrannazar.com/GameBoy-Emulation-in-JavaScript:-Sprites

        for index in 0 .. 40 {

            // If we reach max sprite per line exit.
            if sprite_counter >= 10 {
                break;
            }

            let mut is_rendered = false;
            let sprite_oam: SpriteOam = self.get_sprite_attributes(index);

            // is y out of bounds
            if display_y < sprite_oam.y_cord || display_y >= sprite_oam.y_cord + sprite_size { continue }

            // is x out of bounds
            if sprite_oam.x_cord < (-7) || sprite_oam.x_cord >= SCREEN_W as i32 { continue }

            let sprite_tile = self.get_sprite_tile_at_y(&sprite_oam, display_y);

            for x in 0 .. 8i32 {
                is_rendered = true;

                let sprite_x_cord = sprite_oam.x_cord + x;
                let sprite_y_cord = display_y;

                if sprite_x_cord < 0 || sprite_x_cord >= SCREEN_W as i32 {
                    continue;
                }

                let bit_mask = 1 << (if sprite_oam.x_flip { x } else { 7 - x } as u32);
                let palette_index =
                    (if sprite_tile.tile_1 & bit_mask != 0 {1} else {0}) |
                    (if sprite_tile.tile_2 & bit_mask != 0 {2} else {0});

                if palette_index == 0 {
                    continue
                }

                let priority = self.scanline_priority[sprite_x_cord as usize];

                if self.model == GameboyType::COLOR {
                    // Priority
                    // Bit 7 of the sprite attribute flags determines that a sprite will appear above the background & window if it is 0
                    // below them both (visible only through colo(u)r 0 of background & window) if it is 1.
                    // When bit 7 of bank 1 of tile attribute memory ($9800-$9fff) is set to 1 it will force the background and/or window to have priority over the sprite attribute flags and to appear over sprites no matter what the setting of the sprite attribute flags.
                    // If bit 0 of register LCDC ($ff40) is 0 then sprites will always appear above the background & window regardless of the settings of sprite attribute flags & tile attribute memory.

                    if self.obj_master_priority {
                        // render
                    } else if !self.lcd_display_enable {
                        // render
                    } else if priority == PriorityType::BgPriority {
                        continue;
                    } else if sprite_oam.has_priority || priority == PriorityType::BgColor0 {
                        // render
                    } else {
                        continue;
                    }

                    let palette = self.cbg_obj[sprite_oam.palette_number as usize][palette_index];

                    let r = palette[0];
                    let g = palette[1];
                    let b = palette[2];

                    self.set_rgb_at(sprite_x_cord as usize, sprite_y_cord as usize, r, g, b);
                } else {
                    if !sprite_oam.has_priority && priority != PriorityType::BgColor0 {
                        continue;
                    }

                    let palette = if sprite_oam.pal_palette_index == 1 { self.pal_obj_palette_1 } else { self.pal_obj_palette_0 };

                    let r = palette[palette_index];
                    let g = palette[palette_index];
                    let b = palette[palette_index];

                    self.set_rgb_at(sprite_x_cord as usize, sprite_y_cord as usize, r, g, b);
                }

            }
            if is_rendered {
                sprite_counter += 1;
            }
        }
    }

    fn get_sprite_tile_at_y(&self, oam: &SpriteOam, y: i32) -> TileData {
        // Specifies the sprites Tile Number (00-FF). This (unsigned) value selects a tile from memory at 8000h-8FFFh.
        // In CGB Mode this could be either in VRAM Bank 0 or 1, depending on Bit 3 of the following byte.

        let tile_x: u16 = if oam.y_flip {
            (self.sprite_size - 1 - (y - oam.y_cord)) as u16
        } else {
            (y - oam.y_cord) as u16
        } as u16;

        let tile_y: u16 = oam.tile_number;

        let tile_address = 0x8000u16 + (tile_y * 16) + (tile_x * 2);

        let tile_1 = self.read_byte_from_vram(oam.vram_bank, tile_address);
        let tile_2 = self.read_byte_from_vram(oam.vram_bank, tile_address + 1);

        return TileData {
            tile_1,
            tile_2,
        }
    }

    fn get_sprite_attributes(&mut self, index: u16) -> SpriteOam {
        // GameBoy video controller can display up to 40 sprites either in 8x8 or in 8x16 pixels.
        // Sprite attributes reside in the Sprite Attribute Table (OAM - Object Attribute Memory) at $FE00-FE9F.
        // Each of the 40 entries consists of four bytes.

        let address = 0xFE00 + (index * 4);

        let y_cord = self.read_byte(address + 0) as i32 - 16;
        let x_cord = self.read_byte(address + 1) as i32 - 8;

        // Specifies the sprites Tile Number (00-FF). This (unsigned) value selects a tile from memory at 8000h-8FFFh.
        // In CGB Mode this could be either in VRAM Bank 0 or 1, depending on Bit 3 of the following byte.
        // In 8x16 mode, the lower bit of the tile number is ignored. Ie. the upper 8x8 tile is "NN AND FEh",
        // and the lower 8x8 tile is "NN OR 01h".

        let tile_number = (self.read_byte(address + 2) & (if self.sprite_size == 16 {0xFE} else {0xFF})) as u16;
        let flags = self.read_byte(address + 3) as usize;

        //   Bit2-0 Palette number  **CGB Mode Only**     (OBP0-7)
        //   Bit3   Tile VRAM-Bank  **CGB Mode Only**     (0=Bank 0, 1=Bank 1)
        //   Bit4   Palette number  **Non CGB Mode Only** (0=OBP0, 1=OBP1)
        //   Bit5   X flip          (0=Normal, 1=Horizontally mirrored)
        //   Bit6   Y flip          (0=Normal, 1=Vertically mirrored)
        //   Bit7   OBJ-to-BG Priority (0=OBJ Above BG, 1=OBJ Behind BG color 1-3)
        //          (Used for both BG and Window. BG color 0 is always behind OBJ)

        let palette_number: u8 = (flags & 0x07) as u8;
        let vram_bank: u8 = if self.model == GameboyType::CLASSIC {0} else if flags & 0x8 != 0 {1} else {0};
        let pal_palette_index = if flags & 0x10 != 0 {1u8} else {0u8};
        let x_flip = flags & 0x20 != 0;
        let y_flip = flags & 0x40 != 0;
        let has_priority = flags & 0x80 == 0;

        return SpriteOam{
            y_cord,
            x_cord,
            tile_number,
            x_flip,
            y_flip,
            has_priority,
            palette_number,
            vram_bank,
            pal_palette_index
        };
    }

    fn read_byte_from_vram(&self, bank: u8, address: u16) -> u8 {
        return if bank == 0 {
            if address < 0x8000 || address >= 0xA000 { panic!("error"); }
            self.vram[address as usize & 0x1FFF]
        } else {
            if address < 0x8000 || address >= 0xA000 { panic!("error"); }
            self.vram[0x2000 + (address as usize & 0x1FFF)]
        }
    }

    fn set_rgb_at(&mut self, x: usize, y: usize, red: u8, green: u8, blue: u8) {
        let base = (y as usize * SCREEN_W * 3) + (x * 3);

        self.frame[base + 0] = red;
        self.frame[base + 1] = green;
        self.frame[base + 2] = blue;
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
            self.interrupt_flags |= INTERRUPT_TIMER_MASK;
        }
        if self.mode == GpuMode::HBlank && self.mode_0_interrupt {
            self.interrupt_flags |= INTERRUPT_TIMER_MASK;
        }
        if self.mode == GpuMode::VBlank && self.mode_1_interrupt {
            self.interrupt_flags |= INTERRUPT_TIMER_MASK;
        }
    }

    fn update_interrupt_for_lyc(&mut self) {
        if self.lyc_interrupt_enable {
            if self.ly == self.lyc {
                self.interrupt_flags |= INTERRUPT_TIMER_MASK;
            }
        }
    }

    fn update_pal_palettes(&mut self) {
        for i in 0 .. 4 {
            self.pal_bg_palette[i] = self.get_pal_color(self.pal_bg_palette_data, i);
            self.pal_obj_palette_0[i] = self.get_pal_color(self.pal_obj_palette_0_data, i);
            self.pal_obj_palette_1[i] = self.get_pal_color(self.pal_obj_palette_1_data, i);
        }
    }

    fn get_pal_color(&self, value: u8, index: usize) -> u8 {
        match (value >> 2 * index) & 0x03 {
            0 => 255,
            1 => 192,
            2 => 96,
            _ => 0
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x8000 ..= 0x9FFF => self.vram[(self.vram_bank * 0x2000) | (address as usize & 0x1FFF)],
            0xFE00 ..= 0xFE9F => self.voam[address as usize - 0xFE00],
            0xFF40 => {
                (if self.lcd_display_enable { 0x80 } else { 0 }) |
                    (if self.window_tile_map_select == 0x9C00 { 0x40 } else { 0 }) |
                    (if self.window_display_enable { 0x20 } else { 0 }) |
                    (if self.bg_tile_data_select == 0x8000 { 0x10 } else { 0 }) |
                    (if self.bg_tile_map_select == 0x9C00 { 0x08 } else { 0 }) |
                    (if self.sprite_size == 16 { 0x04 } else { 0 }) |
                    (if self.sprite_enable { 0x02 } else { 0 }) |
                    (if self.bg_display_enable { 0x01 } else { 0 })
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
            0xFF47 => self.pal_bg_palette_data,
            0xFF48 => self.pal_obj_palette_0_data,
            0xFF49 => self.pal_obj_palette_1_data,
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
            0xFF6C => 0x0,
            _ => panic!("invalid"),
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x8000 ..= 0x9FFF => self.vram[(self.vram_bank * 0x2000) | (address as usize & 0x1FFF)] = value,
            0xFE00 ..= 0xFE9F => self.voam[address as usize - 0xFE00] = value,
            0xFF40 => {
                let last_lcd_display_enable = self.lcd_display_enable;
                let last_bg_display_enable = self.bg_display_enable;

                self.lcd_display_enable = value & 0x80 == 0x80;
                self.window_tile_map_select = if value & 0x40 == 0x40 { 0x9C00 } else { 0x9800 };
                self.window_display_enable = value & 0x20 == 0x20;
                self.bg_tile_data_select = if value & 0x10 == 0x10 { 0x8000 } else { 0x8800 };
                self.bg_tile_map_select = if value & 0x08 == 0x08 { 0x9C00 } else { 0x9800 };
                self.sprite_size = if value & 0x04 == 0x04 { 16 } else { 8 };
                self.sprite_enable = value & 0x02 == 0x02;
                self.bg_display_enable = value & 0x01 == 0x01;

                if last_lcd_display_enable && !self.lcd_display_enable {
                    self.mode = GpuMode::HBlank;
                    self.ly = 0;
                    self.clock = 0;
                }

                if self.model == GameboyType::COLOR {
                    if last_bg_display_enable && !self.bg_display_enable {
                        self.obj_master_priority = true;
                    }
                }
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
            0xFF47 => { self.pal_bg_palette_data = value; self.update_pal_palettes(); },
            0xFF48 => { self.pal_obj_palette_0_data = value; self.update_pal_palettes(); },
            0xFF49 => { self.pal_obj_palette_1_data = value; self.update_pal_palettes(); },
            0xFF4A => self.window_y_coord = value,
            0xFF4B => self.window_x_coord = value,
            0xFF4C => {}, // i dunno
            0xFF4D => {}, // i dunno
            0xFF4F => self.vram_bank = (value & 0x01) as usize,
            0xFF4E => {}, // i dunno
            0xFF68 => { self.cbg_bg_palette_index = value & 0x3F; self.cbg_bg_palette_increment = value & 0x80 == 0x80; },
            0xFF69 => {
                self.update_palette(PaletteType::BACKGROUND, value);

                if self.cbg_bg_palette_increment { self.cbg_bg_palette_index = (self.cbg_bg_palette_index + 1) & 0x3F; };
            },
            0xFF6A => { self.cbg_obj_index = value & 0x3F; self.cbg_obj_increment = value & 0x80 == 0x80; },
            0xFF6B => {
                self.update_palette(PaletteType::OBJECTS, value);

                if self.cbg_obj_increment { self.cbg_obj_index = (self.cbg_obj_index + 1) & 0x3F; };
            },
            0xFF6C => {}
            _ => panic!("invalid {}", address),
        }
    }

    fn update_palette(&mut self, palette_type: PaletteType, palette_value: u8) {
        // To get the full color GB requires two writes (two bytes)
        // Bit 0-4   Red Intensity   (00-1F)
        // Bit 5-9   Green Intensity (00-1F)
        // Bit 10-14 Blue Intensity  (00-1F)

        let mut pal_num: usize;
        let mut col_num: usize;
        let mut palette: &mut [u8];
        let mut byte_index;

        if palette_type == PaletteType::BACKGROUND {
            pal_num = (self.cbg_bg_palette_index >> 3) as usize;
            col_num = ((self.cbg_bg_palette_index >> 1) & 0x03) as usize;
            palette = &mut self.cbg_bg_palette[pal_num][col_num];
            byte_index = if self.cbg_bg_palette_index & 0x01 == 0x00 {0} else {1};
        } else {
            pal_num = (self.cbg_obj_index >> 3) as usize;
            col_num = ((self.cbg_obj_index >> 1) & 0x03) as usize;
            palette = &mut self.cbg_obj[pal_num][col_num];
            byte_index = if self.cbg_obj_index & 0x01 == 0x00 {0} else {1};
        }

        const RED: usize = 0;
        const GREEN: usize = 1;
        const BLUE: usize = 2;

        if byte_index == 0 {
            // First write
            palette[RED] = palette_value & 0x1F;
            palette[GREEN] = (palette[GREEN] & 0x18) | (palette_value >> 5) ;
        } else {
            // Second write
            palette[GREEN] = (palette[GREEN] & 0x07) | ((palette_value & 0x3) << 3) & 0x1F;
            palette[BLUE] = (palette_value >> 2) & 0x1F;
        }

        if byte_index == 1 {
            // all colors are in range of 0x00 - 0x1F
            // need logic to transform it to 00 - 255

            palette[RED] *= 8;
            palette[GREEN] *= 8;
            palette[BLUE] *= 8;
        }
    }

    pub fn set_gameboy_type(&mut self, model: GameboyType) {
        self.model = model;
    }

}
