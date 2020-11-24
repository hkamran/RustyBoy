mod cpu;
mod bus;
mod console;
mod ppu;
mod cartridge;
mod io;

use crate::console::{Console};

fn main() {

    let console: Console = Console::new();
    console.tick();
}