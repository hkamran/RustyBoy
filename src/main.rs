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

extern crate minifb;


use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

use crate::console::{Console};

fn main() {
    let mut console: Console = Console::new();
    let cart_path = "./Tetris.gb";

    console.load_cartridge(cart_path);
    console.tick();

}
