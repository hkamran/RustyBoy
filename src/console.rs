use crate::cpu::Cpu;
use crate::mmu::Mmu;
use crate::ppu::Ppu;
use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
pub struct Console<> {
    pub cpu: Cpu,
    pub mmu: Mmu,
    pub ppu: Rc<RefCell<Ppu>>,
    // pub cartridge: Box<Cartridge<'a>>,
    // pub io: Box<Io<'a>>,
}

impl Console {

    pub fn new() -> Self {
        let ppu = Rc::new(RefCell::new(Ppu::new()));
        return Console {
            mmu: Mmu::new(ppu.clone()),
            cpu: Cpu::new(),
            ppu
        }
    }

    pub fn tick(&mut self) -> () {
        self.cpu.tick(&mut self.mmu);

        for _x in 0..3 {
            self.ppu.borrow_mut().tick(&mut self.mmu);
        }
    }

}