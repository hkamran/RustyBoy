use crate::cpu::Cpu;
use crate::cartridge::Cartridge;
use crate::ppu::Ppu;
use crate::io::Io;

#[allow(unused)]
pub struct Bus<'a> {
    cpu: Option<&'a Cpu<'a>>,
    ppu: Option<&'a Ppu<'a>>,
    cartridge: Option<&'a Cartridge<'a>>,
    io: Option<&'a Io<'a>>
}

#[allow(unused)]
impl<'a> Bus<'a> {

    pub fn new() -> Self {
        return Bus {
            cpu: None,
            ppu: None,
            cartridge: None,
            io: None
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

    pub fn connect(&mut self, cpu: &'a Cpu, ppu: &'a Ppu, io: &'a Io, cartridge: &'a Cartridge) {
        self.cpu = Option::from(cpu);
        self.ppu = Option::from(ppu);
        self.io = Option::from(io);
        self.cartridge = Option::from(cartridge);
    }

}