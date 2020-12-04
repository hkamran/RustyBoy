use crate::bus::Bus;
use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
pub struct Ppu {
    pub id: u8,
    bus: Rc<RefCell<Bus>>
}

#[allow(dead_code)]
impl Ppu {

    pub fn new(bus: Rc<RefCell<Bus>>) -> Self {
        return Ppu {
            id: 0,
            bus,
        };
    }

    #[allow(unused)]
    pub fn tick(&mut self) -> () {
        println!("Ppu ticked");
    }
}