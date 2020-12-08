use std::fs;
use std::path::Path;
use std::convert::TryInto;
use std::fmt;


pub trait Cartridge {
    fn new(content: &[u8]) -> Self where Self: Sized;
    fn rom_dump(&self, f: &mut fmt::Formatter) -> fmt::Result;
    fn read_byte(&self, addr: u16) -> u8;
    fn write_byte(&mut self, addr: u16, value: u8) -> ();
}

impl fmt::Debug for Cartridge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.rom_dump(f)
    }
}

#[derive(Debug)]
pub struct MBC0 {
    rom: [u8; 0x8000]
}

pub struct MBC1 {
    data: [u8; 0x8000],
    rom_bank: [u8; 0x2000],
    ram_bank: [u8; 0x2000],
}

#[allow(dead_code)]
pub struct MBC2 {
    data: [u8; 0x8000],
    rom_bank: [u8; 0x4000],
    ram_bank: [u8; 0x3000],
}

#[allow(dead_code)]
pub struct MBC3 {
    data: [u8; 0x8000],
    rom_bank: [u8; 0x4000],
    ram_bank: [u8; 0x3000],
}


impl Cartridge for MBC0 {

    fn new(content: &[u8]) -> Self {
        MBC0 { rom: content.try_into().expect("yabe")}
    }

    fn read_byte(&self, addr: u16) -> u8 {
        self.rom[addr as usize]
    }

    fn write_byte(&mut self, addr: u16, value: u8) {
        self.rom[addr as usize] = value;
    }

    fn rom_dump(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:x?}", self.rom)
    }
}


pub fn load(file: &str) -> Box<dyn Cartridge> {
    let path = Path::new(file);
    let content : Vec<u8> = fs::read(path).expect("yabe");

    // Check header to determine type
    let cartridge = match content[0x0147] {
        0x00 => MBC0::new(&content[..]),
        //0x01..=0x03 => MBC1::new(&content[..]),
        //0x05..=0x06 => MBC2::new(&content[..]),
        //0x0F..=0x13 => MBC3::new(&content[..]),
        _ => { panic!("no cartridge type exists");}
    };

    Box::new(cartridge)
}

