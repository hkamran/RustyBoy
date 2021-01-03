use crate::timer::Timer;
use wasm_bindgen::prelude::*;

static MAX_VOLUME: u16 = 8000;
static TWO_DECI: f32 = 0.8;
static VOLUME_LEVELS: usize = 0xF;

//http://www.codeslinger.co.uk/pages/projects/mastersystem/sound.html
#[wasm_bindgen]
pub struct Psg {
    // Each channel has 5 registers
    timer: Timer,
    channel: [[u8; 5]; 4],
    nr50: u8,
    nr51: u8,
    nr52: u8,
    volumes: [f32; 0xF],
    wave_table: [u8; 0xF]
}

#[wasm_bindgen]
impl Psg {
    pub fn new(timer: Timer) -> Self {
        return Psg {
            timer: timer,
            channel: [
                [0x0F; 5], // channel 0: Tone and Sweep
                [0x00; 5], // channel 1: Tone
                [0x00; 5], // channel 2: Wave
                [0x00; 5], // channel 3: Noise
            ],
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

    pub fn get_volume(&self) -> u8 {
        0
    }

    pub fn output(&self) -> u16 {
        //calculate volume
        0
    }

    // https://gbdev.gg8.se/wiki/articles/Gameboy_sound_hardware#Trigger_Event
    fn trigger(&self, index: usize) {
        // do nothing if trigger is not enabled
        let channel = self.channel[index];
        let length; let volume; let period;
        if ! (channel[4] & 0x40) == 0x40 { return };

        match index {
            0|1|3 => {
                length = channel[1] & 0x3F;
                volume = channel[1] & 0xF0 >> 4;
                period = channel[2] & 0x7;
            },
            2 => {
                length = channel[1];
                volume = channel[1] & 0x60 >> 4;
                period = channel[2] & 0x7;
            },
            _ => panic!("yabe")
        };

        //length will determine start and end time in js
    }



    pub fn write_byte(&mut self, address: u16, value: u8) {
        let reg_i: usize = (address & 0x000F % 0x5) as usize;
        match address {
            0xFF10..=0xFF14 => { self.channel[0][reg_i] = value; if ( reg_i == 0x4 ) {self.trigger(0)}},
            0xFF15..=0xFF19 => { self.channel[1][reg_i] = value; if ( reg_i == 0x4 ) {self.trigger(1)}},
            0xFF1A..=0xFF1E => { self.channel[2][reg_i] = value; if ( reg_i == 0x4 ) {self.trigger(2)}},
            0xFF1F..=0xFF23 => { self.channel[3][reg_i] = value; if ( reg_i == 0x4 ) {self.trigger(3)}},
            0xFF24 => { self.nr50 = value },
            0xFF25 => { self.nr51 = value },
            0xFF26 => { self.nr52 = value },
            0xFF30..=0xFF3F => { self.wave_table[ (address & 0x000F) as usize] = value },
            _ => {},
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        let reg_i: usize = (address & 0x000F % 0x5) as usize;
        match address {
            0xFF10..=0xFF14 => { self.channel[0][reg_i] },
            0xFF15..=0xFF19 => { self.channel[1][reg_i] },
            0xFF1A..=0xFF1E => { self.channel[2][reg_i] },
            0xFF1F..=0xFF23 => { self.channel[3][reg_i] },
            0xFF24 => { self.nr50 },
            0xFF25 => { self.nr51 },
            0xFF26 => { self.nr52 },
            0xFF30..=0xFF3F => { self.wave_table[(address & 0x000F) as usize] },
            _ => { 0 },
        }
    }
}
