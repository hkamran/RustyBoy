use crate::cpu::Cpu;
use crate::ppu::Ppu;
use crate::mmu::Mmu;

pub enum Mode {
    COLOR,
    CLASSIC
}

#[allow(dead_code)]
pub struct Console {
    pub cpu: Cpu,
    pub mmu: Mmu,
}

impl Console {

    pub fn new() -> Self {
        return Console {
            mmu: Mmu::new(),
            cpu: Cpu::new()
        }
    }

    pub fn tick(&mut self) -> () {
        self.cpu.tick(&mut self.mmu);

        for _x in 0..3 {
            self.mmu.tick();
        }
    }

}
