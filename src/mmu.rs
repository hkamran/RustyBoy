use crate::cartridge::Cartridge;
use crate::ppu::Ppu;
use crate::dma::{Dma, perform_oam_dma};
use crate::timer::Timer;

#[allow(unused)]
pub struct Mmu {
    zram: [u8; 0x7F],
    wram: [u8; 0x8000],
    wram_bank: usize,
    speed: Speed,
    switch_speed: bool,
    pub interrupt_enable: u8,
    pub interrupt_flag: u8,
    pub ppu: Ppu,
    pub cartridge: Option<Box<dyn Cartridge>>,
    pub dma: Dma,
    pub timer: Timer,
}

#[derive(PartialEq)]
pub enum Speed {
    FAST, SLOW
}

#[allow(unused)]
impl Mmu {

    pub fn new() -> Self {
        return Mmu {
            cartridge: Option::None,
            wram: [0; 0x8000],
            wram_bank: 1,
            zram: [0; 0x7F],
            speed: Speed::SLOW,
            switch_speed: false,
            interrupt_flag: 0,
            interrupt_enable: 0,

            ppu: Ppu::new(),
            dma: Dma::new(),
            timer: Timer::new(),
        };
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000 ..= 0x7FFF => { match &self.cartridge { Some(c) => c.read_byte(address), None => 0 } },
            0x8000 ..= 0x9FFF => { self.ppu.read_byte(address) },
            0xA000 ..= 0xBFFF => { 0 },
            0xC000 ..= 0xCFFF | (0xE000 ..= 0xEFFF) => { self.wram[address as usize & 0x0FFF] },
            0xD000 ..= 0xDFFF | (0xF000 ..= 0xFDFF) => { self.wram[(self.wram_bank * 0x1000) | address as usize & 0x0FFF] },
            0xFE00 ..= 0xFE9F => { self.ppu.read_byte(address) },
            0xFF00 ..= 0xFF00 => { 0 }, // keyboard
            0xFF01 ..= 0xFF02 => { 0 }, // self.serial.read_byte(address)
            0xFF04 ..= 0xFF07 => { self.timer.read_byte(address) }, //
            0xFF0F => { self.interrupt_flag },
            0xFF10 ..= 0xFF3F => { 0 }, // sound
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
            0x0000 ..= 0x7FFF => { match &mut self.cartridge { Some(c) => c.write_byte(address, value), None => () } },
            0x8000 ..= 0x9FFF => { self.ppu.write_byte(address, value) },
            0xA000 ..= 0xBFFF => {},
            0xC000 ..= 0xCFFF | (0xE000 ..= 0xEFFF) => { self.wram[address as usize & 0x0FFF] = value },
            0xD000 ..= 0xDFFF | (0xF000 ..= 0xFDFF) => { self.wram[(self.wram_bank * 0x1000) | (address as usize & 0x0FFF)] = value },
            0xFE00 ..= 0xFE9F => { self.ppu.write_byte(address, value) },
            0xFF00 => {},            // keyboard
            0xFF01 ..= 0xFF02 => {}, // self.serial.write_byte
            0xFF04 ..= 0xFF07 => { self.timer.write_byte(address, value) }, //
            0xFF10 ..= 0xFF3F => {}, // sound
            0xFF46 => { perform_oam_dma(self, value) },
            0xFF4D => { if value & 0x1 == 0x1 { self.switch_speed = true; } },
            0xFF40 ..= 0xFF4F => { self.ppu.write_byte(address, value) },
            0xFF51 ..= 0xFF55 => { self.dma.write_byte(address, value)},
            0xFF68 ..= 0xFF6B => { self.ppu.write_byte(address, value)},
            0xFF0F => { self.interrupt_flag = value },
            0xFF70 ..= 0xFF70 => { self.wram_bank = match value & 0x7 { 0 => 1, n => n as usize }; },
            0xFF80 ..= 0xFFFE => { self.zram[address as usize & 0x007F] = value },
            0xFFFF ..= 0xFFFF => { self.interrupt_enable = value },
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

    pub fn tick(&mut self) -> u32 {
        let cpu_divider = match self.speed {
            Speed::SLOW => 1,
            Speed::FAST => 2,
        };

        let dma_ticks = 0; //self.dma.tick(self);
        let gpu_ticks = cpu_divider + dma_ticks;
        let cpu_ticks = dma_ticks * cpu_divider;

        // for x in 0 .. cpu_ticks {
        //     self.timer.tick();
        // }
        self.interrupt_flag |= self.timer.interrupt;
        self.timer.interrupt = 0;

        // for x in 0 .. gpu_ticks {
        //     self.ppu.tick(self);
        // }
        self.interrupt_flag |= self.ppu.interrupt;
        self.ppu.interrupt = 0;

        return 0;
    }

}
