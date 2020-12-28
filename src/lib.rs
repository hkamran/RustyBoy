mod cpu;
mod mmu;
mod console;
mod ppu;
mod cartridge;
mod io;
mod operations;
mod dma;
mod timer;
mod logger;
mod joypad;

extern crate serde_json;
extern crate wasm_bindgen;

#[macro_use]
extern crate serde_derive;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Logging
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

//static mut CONSOLE: Console = Console::new();

#[wasm_bindgen(start)]
pub fn load_cartridge() -> Result<(), JsValue> {
    //let bytes: Vec<u8> = value.into_serde().unwrap();
    //CONSOLE.load(bytes);

    Ok(())
}
