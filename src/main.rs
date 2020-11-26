mod cpu;
mod bus;
mod console;
mod ppu;
mod cartridge;
mod io;

use crate::console::{Console};

fn main() {

    let mut console: Console = Console::new();
    console.tick();
}