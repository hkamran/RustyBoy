use crate::bus::Bus;
use crate::cpu::registers::Registers;

pub mod addressing;
pub mod operations;
pub mod registers;

pub struct Cpu {
    pub registers: Registers,
    pub bus: Bus

}

impl Cpu {

    pub fn new() -> Self {
        return Cpu {
            registers: Registers::new(),
            bus: Bus::new(),
        }
    }

    pub fn tick(&self) -> () {
        println!("Cpu ticked");
    }

}

