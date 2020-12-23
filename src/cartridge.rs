use std::fs;
use std::path::Path;
use std::convert::TryInto;
use std::fmt;
use crate::console::GameboyType;


pub trait Cartridge {
    fn new(content: &[u8]) -> Self where Self: Sized;
    fn rom_dump(&self, f: &mut fmt::Formatter) -> fmt::Result;
    fn read_byte(&self, addr: u16) -> u8;
    fn write_byte(&mut self, addr: u16, value: u8) -> ();
    fn get_gameboy_type(&self) -> GameboyType;
}

impl fmt::Debug for dyn Cartridge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.rom_dump(f)
    }
}

#[derive(Debug)]
pub struct MBC0 {
    rom: [u8; 0x8000],
}

pub struct MBC1 {
    rom: Vec<u8>,
    rom_bank: usize,
    
    ram: Vec<u8>,
    ram_on: bool,
    ram_mode: bool,
    ram_bank: usize,
}

pub struct MBC5 {
    rom: Vec<u8>,
    rom_bank: usize,

    ram: Vec<u8>,
    ram_on: bool,
    ram_mode: bool,
    ram_bank: usize,
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
        MBC0 {
            rom: content.try_into().expect("yabe"),
        }
    }

    fn rom_dump(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:x?}", self.rom)
    }

    fn read_byte(&self, addr: u16) -> u8 {
        self.rom[addr as usize]
    }

    fn write_byte(&mut self, addr: u16, value: u8) {
        return;
    }

    fn get_gameboy_type(&self) -> GameboyType {
        let mode_byte = self.rom[0x143];
        return if mode_byte == 0x80 || mode_byte == 0xc0 { GameboyType::COLOR } else { GameboyType::CLASSIC };
    }
}

impl Cartridge for MBC1 {

    fn new(content: &[u8]) -> Self {
        MBC1 {
            rom: content.try_into().expect("yabe"),
            rom_bank: 1,
            ram: vec![],
            ram_on: false,
            ram_mode: false,
            ram_bank: 0
        }
    }

    fn rom_dump(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:x?}", self.rom)
    }

    fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0xA000 ..= 0xBFFF => {
                if !self.ram_on { return 0 }
                let ram_bank = if self.ram_mode { self.ram_bank } else { 0 };
                self.ram[(ram_bank * 0x2000) | ((addr & 0x1FFF) as usize)]
            }
            _ => {
                let index = if addr < 0x4000 { addr as usize }
                else  { self.rom_bank  * 0x4000 | ((addr as usize) & 0x3FFF) };

                return *self.rom.get(index).unwrap_or(&0);
            }
        }
    }

    fn write_byte(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000 ..= 0x1FFF => { self.ram_on = value == 0x0A; },
            0x2000 ..= 0x3FFF => {
                self.rom_bank = (self.rom_bank & 0x60) | match (value as usize) & 0x1F { 0 => 1, n => n }
            },
            0x4000 ..= 0x5FFF => {
                if !self.ram_mode {
                    self.rom_bank = self.rom_bank & 0x1F | (((value as usize) & 0x03) << 5)
                } else {
                    self.rom_bank = (value as usize) & 0x03;
                }
            },
            0x6000 ..= 0x7FFF => { self.ram_mode = (value & 0x01) == 0x01; },
            0xA000 ..= 0xBFFF => {
                if !self.ram_on { return }
                let ram_bank = if self.ram_mode { self.ram_bank } else { 0 };
                self.ram[(ram_bank * 0x2000) | ((addr & 0x1FFF) as usize)] = value;
            }
            _ => panic!("error"),
        }
        self.rom[addr as usize] = value;
    }

    fn get_gameboy_type(&self) -> GameboyType {
        let mode_byte = self.rom[0x143];
        return if mode_byte == 0x80 || mode_byte == 0xc0 { GameboyType::COLOR } else { GameboyType::CLASSIC };
    }
}

impl Cartridge for MBC5 {

    fn new(content: &[u8]) -> Self {
        MBC5 {
            rom: content.try_into().expect("yabe"),
            rom_bank: 1,
            ram: ::std::iter::repeat(0u8).take(0x1E).collect(),
            ram_bank: 0,
            ram_on: false,
            ram_mode: false,
        }
    }

    fn rom_dump(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:x?}", self.rom)
    }

    fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            0xA000 ..= 0xBFFF => {
                if !self.ram_on { return 0 }
                let ram_bank = self.ram_bank;
                self.ram[(ram_bank * 0x2000) | ((addr & 0x1FFF) as usize)]
            }
            _ => {
                let index = if addr < 0x4000 { addr as usize }
                else  { self.rom_bank  * 0x4000 | ((addr as usize) & 0x3FFF) };

                return *self.rom.get(index).unwrap_or(&0);
            }
        }
    }

    fn write_byte(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000 ..= 0x1FFF => { self.ram_on = value == 0x0A; },
            0x2000 ..= 0x2FFF => self.rom_bank = (self.rom_bank & 0x100) | (value as usize),
            0x3000 ..= 0x3FFF => self.rom_bank = (self.rom_bank & 0x0FF) | (((value & 0x1) as usize) << 8),
            0x4000 ..= 0x5FFF => self.ram_bank = (value & 0x0F) as usize,
            0x6000 ..= 0x7FFF => {}
            0xA000 ..= 0xBFFF => {
                if self.ram_on == false { return }
                self.ram[self.ram_bank * 0x2000 | ((addr as usize) & 0x1FFF)] = value;
            }
            _ => panic!("error"),
        }
        self.rom[addr as usize] = value;
    }

    fn get_gameboy_type(&self) -> GameboyType {
        let mode_byte = self.rom[0x143];
        return if mode_byte == 0x80 || mode_byte == 0xc0 { GameboyType::COLOR } else { GameboyType::CLASSIC };
    }
}

const HEADER_INDEX_FOR_CARTRIDGE_TYPE: usize = 0x0147;

pub fn load_from_file_address(file_path: &str) -> Box<dyn Cartridge> {
    let path = Path::new(file_path);
    let content : Vec<u8> = fs::read(path).expect("yabe");
    return load_from_bytes(content);
}

pub fn load_from_bytes(content: Vec<u8>) -> Box<dyn Cartridge> {
    let cartridge_type = content[HEADER_INDEX_FOR_CARTRIDGE_TYPE];

    if 0x00 <= cartridge_type && cartridge_type <= 0x00 {
        let cartridge = MBC0::new(&content[..]);
        return Box::new(cartridge);
    } else if 0x01 <= cartridge_type && cartridge_type <= 0x03 {
        let cartridge = MBC1::new(&content[..]);
        return Box::new(cartridge);
    } else if 0x05 <= cartridge_type && cartridge_type <= 0x06 {
        panic!("not implemented");
    } else if 0x0F <= cartridge_type && cartridge_type <= 0x13 {
        panic!("not implemented");
    } else if 0x19 <= cartridge_type && cartridge_type <= 0x1E {
        let cartridge = MBC5::new(&content[..]);
        return Box::new(cartridge);
    } else {
        panic!("no cartridge type exists");
    }
}

