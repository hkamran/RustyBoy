use crate::cpu::Cpu;
use crate::bus::Bus;

#[allow(dead_code)]
pub struct Console {
    pub cpu: Cpu,
    pub bus: Bus,
    // pub ppu: Box<Ppu<'a>>,
    // pub cartridge: Box<Cartridge<'a>>,
    // pub io: Box<Io<'a>>,
}

impl Console {

    pub fn new() -> Self {
        return Console {
            bus: Bus::new(),
            cpu: Cpu::new(),
            // ppu,
            // io,
            // cartridge,
        }
    }

    pub fn tick(&mut self) -> () {
        self.cpu.tick(&mut self.bus);

        for _x in 0..3 {
            // &self.ppu.tick();
        }
    }

}