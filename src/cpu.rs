use crate::bus::Bus;

pub mod addressing;
pub mod operations;

pub struct Registers {


}

pub struct Cpu {
    pub registers: Registers,
    pub bus: Bus

}

impl Cpu {

    pub fn new() -> Self {
        return Cpu {
            registers: Registers {},
            bus: Bus::new(),
        }
    }

    pub fn tick(&self) -> () {
        println!("Cpu ticked");
    }

}

