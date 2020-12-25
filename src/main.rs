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

extern crate minifb;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

use crate::console::{Console};

fn main() {
    let mut console: Console = Console::new();
    let cart_path = "./roms/Tetris.gb";

    console.load(cart_path);
    console.reset();
    console.execute_ticks(3115847);

    console.execute_ticks(1);
    print!("finished")
}
