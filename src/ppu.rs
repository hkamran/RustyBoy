use crate::mmu::Mmu;

#[allow(dead_code)]
pub struct Ppu {
    pub id: u8
}

#[allow(dead_code)]
impl Ppu {

    pub fn new() -> Self {
        return Ppu {
            id: 0
        };
    }

    #[allow(unused)]
    pub fn tick(&mut self, bus: &mut Mmu) -> () {
        println!("Ppu ticked");
    }
}