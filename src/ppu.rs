use crate::bus::Bus;

#[allow(dead_code)]
pub struct Ppu<'a> {
    pub bus: &'a Bus<'a>,
}

impl<'a> Ppu<'a> {

    pub fn new(bus: &'a Bus) -> Self {
        return Ppu {
            bus
        };
    }

    pub fn tick(&self) -> () {
        println!("Ppu ticked");
    }
}