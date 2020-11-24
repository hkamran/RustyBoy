use crate::cpu::{Cpu, Registers};
use crate::bus::{Bus};
use crate::ppu::{Ppu};
use crate::cartridge::{Cartridge};
use crate::io::{Io};

pub struct Console {
    pub cpu: Cpu,
    pub bus: Bus,
    pub ppu: Ppu,
    pub cartridge: Cartridge,
    pub io: Io,
}

impl Console {

    pub fn new() -> Self {
        return Console {
            cpu: Cpu::new(),
            bus: Bus::new(),
            ppu: Ppu::new(),
            cartridge: Cartridge::new(),
            io: Io::new(),
        }
    }

    pub fn tick(&self) -> () {
        &self.cpu.tick();

        for x in 0..3 {
            &self.ppu.tick();
        }
    }

}