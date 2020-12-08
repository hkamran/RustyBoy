mod cpu;
mod mmu;
mod console;
mod cartridge;
mod ppu;
mod io;
mod operations;
mod dma;
mod timer;

use crate::console::{Console};

fn main() {
    let mut console: Console = Console::new();
    let cart_path = "./Tetris.gb";

    console.mmu.load_cartridge(cart_path);
    //println!("{:?}", console.mmu.cartridge);
    console.tick();
}
