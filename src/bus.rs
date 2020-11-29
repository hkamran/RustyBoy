// use crate::cpu::Cpu;
// use crate::cartridge::Cartridge;
// use crate::ppu::Ppu;
// use crate::io::Io;

#[allow(unused)]
pub struct Bus {
}

#[allow(unused)]
impl Bus {

    pub fn new() -> Self {
        return Bus {
        };
    }

    pub fn read_byte(&mut self, addr: u16) -> u8 {
        return 0;
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {

    }

    pub fn read_word(&mut self, addr: u16) -> u16 {
        return 0;
    }

    pub fn write_word(&mut self, addr: u16, value: u16) {

    }

    pub fn push_byte(&mut self) {

    }

    pub fn pop_byte(&mut self) {

    }

}