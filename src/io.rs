use crate::bus::Bus;

#[allow(dead_code)]
pub struct Io<'a> {
    pub bus: &'a Bus<'a>,
}

impl<'a> Io<'a> {

    pub fn new(bus: &'a Bus) -> Self {
        return Io {
            bus
        };
    }

}