extern crate serde_json;
extern crate wasm_bindgen;

use std::fs;
use std::convert::TryInto;
use std::fmt;
use crate::console::GameboyType;
use wasm_bindgen::prelude::*;

pub const HEADER_INDEX_FOR_CARTRIDGE_TYPE: usize = 0x0147;

// Logging
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u8(a: u8);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_usize(a: usize);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}


#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CartridgeType {
    None,
    MBC0,
    MBC1,
    //MBC2,
    //MBC3,
    //MBC4,
    MBC5,
}

fn read_byte(cartridge: &Cartridge, addr: u16) -> u8 {
    match cartridge.cartridge_type {
        CartridgeType::None => 0,
        CartridgeType::MBC0 => { cartridge.rom[addr as usize] },
        CartridgeType::MBC1 | CartridgeType::MBC5 => {
            match addr {
                0xA000 ..= 0xBFFF => {
                    if !cartridge.ram_on { return 0 }
                    let ram_bank = if cartridge.ram_mode { cartridge.ram_bank } else { 0 };
                    cartridge.ram[(ram_bank * 0x2000) | ((addr & 0x1FFF) as usize)]
                }
                _ => {
                    let index = if addr < 0x4000 { addr as usize }
                    else  { cartridge.rom_bank  * 0x4000 | ((addr as usize) & 0x3FFF) };

                    return *cartridge.rom.get(index).unwrap_or(&0);
                }
            }
        }
    }
}

fn write_byte(cartridge: &mut Cartridge, addr: u16, value: u8) {
    match cartridge.cartridge_type {
        CartridgeType::None => {},
        CartridgeType::MBC0 => {},
        CartridgeType::MBC1 => {
            match addr {
                0x0000 ..= 0x1FFF => { cartridge.ram_on = value == 0x0A; },
                0x2000 ..= 0x3FFF => {
                    cartridge.rom_bank = (cartridge.rom_bank & 0x60) | match (value as usize) & 0x1F { 0 => 1, n => n }
                },
                0x4000 ..= 0x5FFF => {
                    if !cartridge.ram_mode {
                        cartridge.rom_bank = cartridge.rom_bank & 0x1F | (((value as usize) & 0x03) << 5)
                    } else {
                        cartridge.rom_bank = (value as usize) & 0x03;
                    }
                },
                0x6000 ..= 0x7FFF => { cartridge.ram_mode = (value & 0x01) == 0x01; },
                0xA000 ..= 0xBFFF => {
                    if !cartridge.ram_on { return }
                    let ram_bank = if cartridge.ram_mode { cartridge.ram_bank } else { 0 };
                    cartridge.ram[(ram_bank * 0x2000) | ((addr & 0x1FFF) as usize)] = value;
                }
                _ => panic!("error"),
            }
            cartridge.rom[addr as usize] = value;
        },
        CartridgeType::MBC5 => {
            match addr {
                0x0000 ..= 0x1FFF => { cartridge.ram_on = value == 0x0A; },
                0x2000 ..= 0x2FFF => cartridge.rom_bank = (cartridge.rom_bank & 0x100) | (value as usize),
                0x3000 ..= 0x3FFF => cartridge.rom_bank = (cartridge.rom_bank & 0x0FF) | (((value & 0x1) as usize) << 8),
                0x4000 ..= 0x5FFF => cartridge.ram_bank = (value & 0x0F) as usize,
                0x6000 ..= 0x7FFF => {}
                0xA000 ..= 0xBFFF => {
                    if cartridge.ram_on == false { return }
                    cartridge.ram[cartridge.ram_bank * 0x2000 | ((addr as usize) & 0x1FFF)] = value;
                }
                _ => panic!("error"),
            }
            cartridge.rom[addr as usize] = value;
        }
    }
}


#[wasm_bindgen]
pub struct Cartridge {
    pub cartridge_type: CartridgeType,
    rom: Vec<u8>,
    rom_bank: usize,
    ram: [u8; 0x50000],
    ram_on: bool,
    ram_mode: bool,
    ram_bank: usize,
}


impl Cartridge {

    pub fn new() -> Self {
        Self {
            rom: vec![0; 1],
            rom_bank: 1,
            ram: [0; 0x50000],
            ram_on: false,
            ram_mode: false,
            ram_bank: 0,
            cartridge_type: CartridgeType::None
        }
    }

    pub fn set_rom(&mut self, rom: Vec<u8>) {
        self.rom = rom;
    }

    pub fn rom_dump(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:x?}", self.rom)
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        read_byte(self, addr)
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        write_byte(self, addr, value);
    }

    pub fn get_gameboy_type(&self) -> GameboyType {
        let mode_byte = self.rom[0x143];
        return if mode_byte == 0x80 || mode_byte == 0xc0 { GameboyType::COLOR } else { GameboyType::CLASSIC };
    }
}
