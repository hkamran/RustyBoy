use crate::mmu::Mmu;
use wasm_bindgen::prelude::*;

static MAX_VOLUME: u16 = 8000;
static TWO_DECI: f32 = 0.8;
static VOLUME_LEVELS: usize = 0xF;

//http://www.codeslinger.co.uk/pages/projects/mastersystem/sound.html
#[wasm_bindgen]
pub struct Psg {
    volumes: [f32; 0xF],
    ch0: u8, // Tone and sweep
    ch1: u8, // Tone
    ch2: u8, // Wave
    ch3: u8, // Noise
}

impl Psg {
    pub fn new() -> Self {
        return Psg {
            ch0: 0xFF,
            ch1: 0x00,
            ch2: 0x00,
            ch3: 0x00,
            volumes : {
                let mut vol_table : [f32; 0xF] = [0.0; 0xF];
                let mut curvol: f32 = MAX_VOLUME as f32;
                for i in 0..VOLUME_LEVELS {
                    vol_table[i] = curvol;
                    curvol *= TWO_DECI;
                }
                vol_table
            }
        }
    }



    pub fn output() -> u16 {
        //calculate volume
        0
   }


}
