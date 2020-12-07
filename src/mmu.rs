use crate::ppu::Ppu;
use crate::dma::Dma;

#[allow(unused)]
pub struct Mmu {
    wram: [u8; 0x8000],
    wram_bank: usize,
    zram: [u8; 0x7F],

    speed: Speed,
    switch_speed: bool,
    pub interrupt_enable: u8,
    pub interrupt_flag: u8,

    pub ppu: Ppu,
    pub dma: Dma,
}

#[derive(PartialEq)]
pub enum Speed {
    FAST, SLOW
}

#[allow(unused)]
impl Mmu {

    pub fn new() -> Self {
        return Mmu {
            wram: [0; 0x8000],
            wram_bank: 1,
            zram: [0; 0x7F],

            speed: Speed::SLOW,
            switch_speed: false,
            interrupt_flag: 0,
            interrupt_enable: 0,

            ppu: Ppu::new(),
            dma: Dma::new(),
        };
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000 ..= 0x7FFF => { 0 },
            0x8000 ..= 0x9FFF => { self.ppu.read_byte(address) },
            0xA000 ..= 0xBFFF => { 0 },
            0xC000 ..= 0xCFFF | (0xE000 ..= 0xEFFF) => { self.wram[address as usize & 0x0FFF] },
            0xD000 ..= 0xDFFF | (0xF000 ..= 0xFDFF) => { self.wram[(self.wram_bank * 0x1000) | address as usize & 0x0FFF] },
            0xFE00 ..= 0xFE9F => { self.ppu.read_byte(address) },
            0xFF00 ..= 0xFF00 => { 0 },
            0xFF01 ..= 0xFF02 => { 0 },//self.serial.read_byte(address)
            0xFF04 ..= 0xFF07 => { 0 }, //self.timer.read_byte(address)
            0xFF0F => { self.interrupt_flag },
            0xFF10 ..= 0xFF3F => { 0 },
            0xFF4D => (if self.speed == Speed::FAST { 0x80 } else { 0 }) | (if self.switch_speed { 1 } else { 0 }),
            0xFF40 ..= 0xFF4F => { self.ppu.read_byte(address) },
            0xFF51 ..= 0xFF55 => { self.dma.read_byte(address) },
            0xFF68 ..= 0xFF6B => { self.ppu.read_byte(address) },
            0xFF70 ..= 0xFF70 => { self.wram_bank as u8 },
            0xFF80 ..= 0xFFFE => { self.zram[address as usize & 0x007F] },
            0xFFFF ..= 0xFFFF => { self.interrupt_enable },
            _ => 0,
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            (0x0000 ..= 0x7FFF) => {},
            (0x8000 ..= 0x9FFF) => {},
            (0xA000 ..= 0xBFFF) => {},
            (0xC000 ..= 0xCFFF) | (0xE000 ..= 0xEFFF) => { self.wram[address as usize & 0x0FFF] = value },
            (0xD000 ..= 0xDFFF) | (0xF000 ..= 0xFDFF) => { self.wram[(self.wram_bank * 0x1000) | (address as usize & 0x0FFF)] = value },
            (0xFE00 ..= 0xFE9F) => {},
            (0xFF00 ..= 0xFF00) => {},
            (0xFF01 ..= 0xFF02) => {},
            (0xFF04 ..= 0xFF07) => {},
            (0xFF10 ..= 0xFF3F) => {},
            (0xFF46 ..= 0xFF46) => {},
            (0xFF4D ..= 0xFF4D) => {},
            (0xFF40 ..= 0xFF4F) => {},
            (0xFF51 ..= 0xFF55) => {},
            (0xFF68 ..= 0xFF6B) => {},
            (0xFF0F ..= 0xFF0F) => {},
            (0xFF70 ..= 0xFF70) => { self.wram_bank = match value & 0x7 { 0 => 1, n => n as usize }; },
            (0xFF80 ..= 0xFFFE) => { self.zram[address as usize & 0x007F] = value },
            (0xFFFF ..= 0xFFFF) => {},
            _ => {},
        };
    }

    pub fn read_word(&self, address: u16) -> u16 {
        let low = (self.read_byte(address) as u16);
        let high  = (self.read_byte(address + 1) as u16);

        return (high << 8) | low;
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        let low = (value & 0xFF) as u8;
        let high = (value >> 8) as u8;

        self.write_byte(address, low);
        self.write_byte(address + 1, high);
    }

    pub fn toggle_speed(&mut self) {
        if self.switch_speed {
            self.speed = if Speed::FAST == self.speed { Speed::SLOW } else { Speed::FAST };
        }
        self.switch_speed = false;
    }

}
