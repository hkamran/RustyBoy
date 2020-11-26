use crate::cpu::{Cpu};
use crate::bus::{Bus};
use crate::ppu::{Ppu};
use crate::cartridge::{Cartridge};
use crate::io::{Io};

#[allow(dead_code)]
pub struct Console<'a> {
    pub cpu: Box<Cpu<'a>>,
    pub ppu: Box<Ppu<'a>>,
    pub cartridge: Box<Cartridge<'a>>,
    pub io: Box<Io<'a>>,
    pub bus: Box<Bus<'a>>,
}

impl<'a> Console<'a> {

    pub fn new() -> Self {

        let mut bus = Box::new(Bus::new());
        let cpu = Box::new(Cpu::new(bus.as_ref()));
        let ppu = Box::new(Ppu::new(bus.as_ref()));
        let io = Box::new(Io::new(bus.as_ref()));
        let cartridge = Box::new(Cartridge::new(bus.as_ref()));

        bus.connect(
            cpu.as_ref(),
            ppu.as_ref(),
            io.as_ref(),
            cartridge.as_ref());

        return Console {
            cpu,
            ppu,
            io,
            cartridge,
            bus,
        }
    }

    pub fn tick(&mut self) -> () {
        &self.cpu.tick();

        for _x in 0..3 {
            &self.ppu.tick();
        }
    }

}