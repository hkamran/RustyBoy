use crate::cpu::Cpu;
use crate::mmu::Mmu;
use crate::ppu::Ppu;

pub enum Mode {
    COLOR,
    CLASSIC
}

#[allow(dead_code)]
pub struct Console {
    pub cpu: Cpu,
    pub bus: Mmu,
    pub ppu: Ppu,
    // pub cartridge: Box<Cartridge<'a>>,
    // pub io: Box<Io<'a>>,
}

impl Console {

    pub fn new() -> Self {
        return Console {
            bus: Mmu::new(),
            cpu: Cpu::new(),
            ppu: Ppu::new(),
            // io,
            // cartridge,
        }
    }

    pub fn tick(&mut self) -> () {
        self.cpu.tick(&mut self.bus);

        for _x in 0..3 {
            self.ppu.tick(&mut self.bus);
        }
    }

}