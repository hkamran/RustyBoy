mod cpu;
mod mmu;
mod console;
mod cartridge;
mod ppu;
mod io;
mod operations;
mod dma;
mod timer;
mod screen;
mod logger;
mod joypad;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

use crate::console::{Console};

fn main() {
    let mut console: Console = Console::new();
    let cart_path = "./roms/cgb-acid2.gbc";

    console.mmu.load_cartridge(cart_path);
    //println!("{:?}", console.mmu.cartridge.as_ref().unwrap().read_byte(0x0147));
    for x in 0 .. 100 {
        log("{}", x);
        console.tick();
    }

    print!("finished")
}
