mod cpu;
mod mmu;
mod console;
mod ppu;
mod cartridge;
mod io;
mod operations;

use crate::console::{Console};

fn main() {
    let mut console: Console = Console::new();
    console.tick();
}