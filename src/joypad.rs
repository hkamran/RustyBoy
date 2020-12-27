use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[derive(PartialEq, Copy, Clone)]
pub struct Joypad {
    row0: u8,
    row1: u8,
    data: u8,
    pub interrupt: u8,
}

impl Joypad {

    pub fn new() -> Joypad {
        Joypad {
            row0: 0x0F,
            row1: 0x0F,
            data: 0xFF,
            interrupt: 0,
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        return self.data
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.data = (self.data & 0xCF) | (value & 0x30);

        let old_values = self.data & 0xF;
        let mut new_values = 0xF;

        if self.data & 0x10 == 0x00 {
            new_values &= self.row0;
        }
        if self.data & 0x20 == 0x00 {
            new_values &= self.row1;
        }

        if old_values == 0xF && new_values != 0xF {
            self.interrupt |= 0x10;
        }

        self.data = (self.data & 0xF0) | new_values;
    }
}
