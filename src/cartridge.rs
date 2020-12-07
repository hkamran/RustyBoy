// use crate::bus::Bus;

#[allow(dead_code)]
pub trait Cartridge {
    pub fn read_byte(addr: u16) -> u8;
    pub fn write_byte(addr: u16, value: u8) -> ();

    pub fn load(file: &str) -> Cartridge {
        let path = Path::new(file);
        let content : Vec<u8> = fs::read(path).expect("yabe");

        // Check header to determine type
        let ctype = content[0x0147];
        match ctype {
            0x01 => MBC0::new(&content[..]),
            0x02 => MBC1::new(&content[..]),
            0x03 => MBC1::new(&content[..]),
            0x04 => MBC1::new(&content[..]),
            0x05 => MBC2::new(&content[..]),
            0x06 => MBC2::new(&content[..]),
            0x07 => MBC2::new(&content[..]),
            0x08 => MBC2::new(&content[..]),
            0x09 => MBC2::new(&content[..]),
            0x0A => MBC2::new(&content[..]),
            0x0B => MBC2::new(&content[..]),
            0x0C => MBC2::new(&content[..]),
            0x0D => MBC2::new(&content[..]),
            0x0E => MBC2::new(&content[..]),
            0x0F => MBC2::new(&content[..]),
            0x11 => MBC2::new(&content[..]),
            0x12 => MBC2::new(&content[..]),
            0x13 => MBC2::new(&content[..]),
            0x14 => MBC2::new(&content[..]),
            0x15 => MBC2::new(&content[..]),
            0x16 => MBC2::new(&content[..]),
            _ => { panic!("no cartridge type exists");}
        }
    }
}

pub struct MBC0 {
    rom: [u8; 0x8000]
}

pub struct MBC1 {
    data: [u8; 0x8000]
    rom_bank: [[u8; 0x4000]],
    ram_bank: [[u8; 0x3000]],
}

pub struct MBC2 {
    data: [u8; 0x8000]
    rom_bank: [[u8; 0x4000]],
    ram_bank: [[u8; 0x3000]],
}

#[allow(dead_code)]
impl MBC0 for Cartridge {
    pub fn new(content: [u8]) -> Self {
        return Cartridge {
            rom: content
        };
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        self.rom[addr]
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        self.rom[addr] = value;
    }

}
