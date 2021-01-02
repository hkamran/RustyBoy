use crate::mmu::Mmu;
use wasm_bindgen::prelude::*;

static MAX_VOLUME: u16 = 8000;
static TWO_DECI: f32 = 0.8;
static VOLUME_LEVELS: usize = 0xF;

//http://www.codeslinger.co.uk/pages/projects/mastersystem/sound.html
#[wasm_bindgen]
pub struct Psg {
    ch0: [u8; 0x5], //channel 0: Tone and sweep
    ch1: [u8; 0x5],// channel 1: Tone
    ch2: [u8; 0x5],// channel 2: Wave
    ch3: [u8; 0x5],// channel 3: Noise
    // Control/ status
    nr50: u8,
    nr51: u8,
    nr52: u8,
    volumes: [f32; 0xF],
    wave_table: [u8; 0xF]
}

impl Psg {
    pub fn new() -> Self {
        return Psg {
            ch0: [0xF; 0x5],
            ch1: [0x0; 0x5],
            ch2: [0x0; 0x5],
            ch3: [0x0; 0x5],
            nr50: 0x00,
            nr51: 0x00,
            nr52: 0x00,
            wave_table: [0x0; 0xF],
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

   pub fn write_byte(&mut self, address: u16, value: u8) {
       let reg_index: usize = (address & 0x000F % 0x5) as usize;
       match address {
           0xFF10..=0xFF14 => { self.ch0[reg_index] = value },
           0xFF10..=0xFF14 => { self.ch1[reg_index] = value },
           0xFF10..=0xFF14 => { self.ch2[reg_index] = value },
           0xFF10..=0xFF14 => { self.ch3[reg_index] = value },
           0xFF24 => { self.nr50 = value },
           0xFF25 => { self.nr51 = value },
           0xFF26 => { self.nr52 = value },
           0xFF30..=0xFF3F => { self.wave_table[ (address & 0x000F) as usize] = value },
           _ => {},
       }
   }

   pub fn read_byte(&self, address: u16) -> u8 {
       let reg_index :usize = (address & 0x000F % 0x5) as usize;
       match address {
           0xFF10..=0xFF14 => { self.ch0[reg_index] },
           0xFF10..=0xFF14 => { self.ch1[reg_index] },
           0xFF10..=0xFF14 => { self.ch2[reg_index] },
           0xFF10..=0xFF14 => { self.ch3[reg_index] },
           0xFF24 => { self.nr50 },
           0xFF25 => { self.nr51 },
           0xFF26 => { self.nr52 },
           0xFF30..=0xFF3F => { self.wave_table[(address & 0x000F) as usize] },
           _ => { 0 },
       }
   }
}
