use crate::mmu::Mmu;

#[allow(dead_code)]
pub struct Ppu {
}

#[allow(dead_code)]
impl Ppu {

    pub fn new() -> Self {
        return Ppu {

        };
    }

    pub fn tick(&mut self, bus: &mut Mmu) -> () {
        println!("Ppu ticked");
    }
}