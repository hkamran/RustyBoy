use wasm_bindgen::prelude::*;

static MAXVOLUME: u16 = 8000;

//http://www.codeslinger.co.uk/pages/projects/mastersystem/sound.html
#[wasm_bindgen]
pub struct Psg {
    ch0: u8,
    ch1: u8,
    ch2: u8,
    ch3: u8,
}

impl Psg {
    pub fn new() -> Self {
        return Psg {
            ch0: 0xF,
            ch1: 0xF,
            ch2: 0xF,
            ch3: 0xF,
        }
    }
}
