use crate::bus::Bus;

#[allow(dead_code)]
pub struct Cartridge<'a> {
    pub bus: &'a Bus<'a>,
}

impl<'a> Cartridge<'a> {

    pub fn new(bus: &'a Bus) -> Self {
        return Cartridge {
            bus
        };
    }

}