use crate::cpu::Cpu;
use crate::bus::Bus;
use crate::ppu::Ppu;
use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
pub struct Console<> {
    pub cpu: Rc<RefCell<Cpu>>,
    pub bus: Rc<RefCell<Bus>>,
    pub ppu: Rc<RefCell<Ppu>>,
    // pub cartridge: Box<Cartridge<'a>>,
    // pub io: Box<Io<'a>>,
}

impl Console {

    pub fn new() -> Self {
        let bus = Rc::new(RefCell::new(Bus::new()));

        let ppu = Rc::new(RefCell::new(Ppu::new(bus.clone())));
        let cpu = Rc::new(RefCell::new(Cpu::new(bus.clone())));

        bus.borrow_mut().ppu = Option::from(ppu.clone());
        bus.borrow_mut().cpu = Option::from(cpu.clone());

        return Console {
            bus: bus.clone(),
            cpu: cpu.clone(),
            ppu: ppu.clone()
        }
    }

    pub fn tick(&mut self) -> () {
        self.cpu.borrow_mut().tick();

        self.bus.borrow_mut().mutate();

        for _x in 0..3 {
            self.ppu.borrow_mut().tick();
        }
    }

}