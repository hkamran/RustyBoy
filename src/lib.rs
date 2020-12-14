mod cpu;
mod mmu;
mod console;
mod ppu;
mod cartridge;
mod io;
mod operations;
mod dma;
mod timer;
mod screen;
mod logger;

use console::Console;
use console_error_panic_hook;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("screen").unwrap();
    let canvas: HtmlCanvasElement = canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();


    let mut console: Console = Console::new();
    let cart_path = "./Tetris.gb";

    console.mmu.load_cartridge(cart_path);
    console.load_canvas_ctx(context);
    //context.beginPath();
    //println!("{:?}", console.mmu.cartridge.as_ref().unwrap().read_byte(0x0147));
    for x in 0 .. 100 {
        println!("{}", x);
        console.tick();
    }

    Ok(())
}
