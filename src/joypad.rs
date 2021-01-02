use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Button {
    RIGHT, LEFT, UP, DOWN, A, B, SELECT, START
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Joypad {
    row0: u8,
    row1: u8,
    data: u8,
    pub interrupt: u8,
}

#[wasm_bindgen]
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
        self.trigger_interrupt();
    }

    pub fn trigger_interrupt(&mut self) {
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

    pub fn press(&mut self, button: Button) {
        match button {
            Button::RIGHT => self.row0 &= !(1 << 0),
            Button::LEFT => self.row0 &= !(1 << 1),
            Button::UP => self.row0 &= !(1 << 2),
            Button::DOWN => self.row0 &= !(1 << 3),
            Button::A => self.row1 &= !(1 << 0),
            Button::B => self.row1 &= !(1 << 1),
            Button::SELECT => self.row1 &= !(1 << 2),
            Button::START => self.row1 &= !(1 << 3),
        }
        self.trigger_interrupt();
    }

    pub fn release(&mut self, button: Button) {
        match button {
            Button::RIGHT => self.row0 |= 1 << 0,
            Button::LEFT => self.row0 |= 1 << 1,
            Button::UP => self.row0 |= 1 << 2,
            Button::DOWN => self.row0 |= 1 << 3,
            Button::A => self.row1 |= 1 << 0,
            Button::B => self.row1 |= 1 << 1,
            Button::SELECT => self.row1 |= 1 << 2,
            Button::START =>self.row1 |= 1 << 3,
        }
        self.trigger_interrupt();
    }
}

