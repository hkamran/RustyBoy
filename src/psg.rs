use crate::timer::Timer;
use wasm_bindgen::prelude::*;

static MAX_VOLUME: u16 = 8000;
static TWO_DECI: f32 = 0.8;
static VOLUME_LEVELS: usize = 0xF;

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
    wave_table: [u8; 0x10]
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
            wave_table: [0x0; 0x10],
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
        let registers = self.channel[index];

        // do nothing if trigger is not enabled
        if ! (registers[4] & 0x40) == 0x40 { return };

        // common flags
        let length; let volume;
        let length_enable = if registers[4] & 0x40 == 0x40 { true } else { false };

        // channel specific flags
        let envelope_add;
        let period;
        let frequency;
        let duty;

        // wave flags
        let dac;

        // square 1 flags
        let sweep;
        let negate;
        let shift;

        // noise flags
        let clock_shift;
        let width_mode;
        let divisor_code;

        match index {
            0 => { // Square 1
                sweep = registers[0] & 0x07;
                negate = if registers[0] & 0x08 == 0x08 { true } else { false };
                shift = registers[0] & 0x07;
                duty = registers[1] & 0xC0 >> 6;
                length = registers[1] & 0x3F;
                volume = registers[2] & 0xF0 >> 4;
                period = registers[2] & 0x7;
                envelope_add = if registers[2] & 0x08 == 0x08 { true } else { false };
                frequency = registers[3] | ((registers[4] & 0x07) << 5);
            },
            1 => { // Square 2
                dac = if registers[0] & 0x80 == 0x80 { true } else { false };
                duty = registers[1] & 0xC0 >> 6;
                length = registers[1] & 0x3F;
                volume = registers[2] & 0xF0 >> 4;
                period = registers[2] & 0x7;
                envelope_add = if registers[2] & 0x08 == 0x08 { true } else { false };
                frequency = registers[3] | ((registers[4] & 0x07) << 5);
            },
            2 => { // Wave
                dac = if registers[0] & 0x80 == 0x80 { true } else { false };
                length = registers[1];
                volume = registers[2] & 0x60 >> 5;
                frequency = registers[3] | ((registers[4] & 0x07) << 5);
            }
            3 => { // Noise
                clock_shift = registers[3] & 0xF0 >> 4;
                width_mode = if registers[3] & 0x80 == 0x80 { true } else { false };
                divisor_code = registers[3] & 0x07;
                length = registers[1] & 0x3F;
                volume = registers[2] & 0xF0 >> 4;
                period = registers[2] & 0x7;
            },
            _ => panic!("invalid channel index type")
        };

        //length will determine start and end time in js
    }



    pub fn write_byte(&mut self, address: u16, value: u8) {
        let reg_i: usize = ((address & 0x000F) % 0x5) as usize;
        //log(format!("addr: {:#X}, value: {:#X}, reg_i: {:#X}", address, value, reg_i).as_str());
        match address {
            0xFF10..=0xFF14 => { self.channel[0][reg_i] = value; if ( reg_i == 0x4 ) {self.trigger(0)}},
            0xFF15..=0xFF19 => { self.channel[1][reg_i] = value; if ( reg_i == 0x4 ) {self.trigger(1)}},
            0xFF1A..=0xFF1E => { self.channel[2][reg_i] = value; if ( reg_i == 0x4 ) {self.trigger(2)}},
            0xFF1F..=0xFF23 => { self.channel[3][reg_i] = value; if ( reg_i == 0x4 ) {self.trigger(3)}},
            0xFF24 => { self.nr50 = value },
            0xFF25 => { self.nr51 = value },
            0xFF26 => { self.nr52 = value },
            0xFF27..=0xFF2F => { } //unused
            0xFF30..=0xFF3F => { self.wave_table[ (address & 0x000F) as usize] = value },
            _ => panic!(format!("invalid memory address {} ", address))
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
            0xFF27..=0xFF2F => { 0 } //unused
            0xFF30..=0xFF3F => { self.wave_table[(address & 0x000F) as usize] },
            _ => panic!(format!("invalid memory address {} ", address))
        }
    }
}
