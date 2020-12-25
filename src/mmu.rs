use crate::cartridge::{Cartridge, CartridgeType, HEADER_INDEX_FOR_CARTRIDGE_TYPE};
use crate::ppu::Ppu;
use crate::dma::{Dma, execute_odma};
use crate::timer::Timer;
use crate::joypad::Joypad;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

#[allow(unused)]
pub struct Mmu {
    zram: [u8; 0x7F],
    sram: [u8; 0x30], // sound
    wram: [u8; 0x8000],
    wram_bank: usize,
    switch_speed: bool,
    pub speed: Speed,
    pub interrupt_enable: u8,
    pub interrupt_flag: u8,
    pub ppu: Ppu,
    pub cartridge: Option<Cartridge>,
    pub dma: Rc<RefCell<Dma>>,
    pub timer: Timer,
    pub joypad: Joypad,
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
            sram: [0; 0x30],
            wram: [0; 0x8000],
            wram_bank: 1,
            zram: [0; 0x7F],
            speed: Speed::SLOW,
            switch_speed: false,
            interrupt_flag: 0,
            interrupt_enable: 0,

            ppu: Ppu::new(),
            dma: Rc::new(RefCell::new(Dma::new())),
            timer: Timer::new(),
            joypad: Joypad::new()
        };
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000 ..= 0x7FFF => { match &self.cartridge { Some(c) => c.read_byte(address), None => 0 } },
            0x8000 ..= 0x9FFF => { self.ppu.read_byte(address) },
            0xA000 ..= 0xBFFF => { match &self.cartridge { Some(c) => c.read_byte(address), None => 0 } },
            0xC000 ..= 0xCFFF | (0xE000 ..= 0xEFFF) => { self.wram[address as usize & 0x0FFF] },
            0xD000 ..= 0xDFFF | (0xF000 ..= 0xFDFF) => { self.wram[(self.wram_bank * 0x1000) | address as usize & 0x0FFF] },
            0xFE00 ..= 0xFE9F => { self.ppu.read_byte(address) },
            0xFF00 ..= 0xFF00 => { self.joypad.read_byte(address) },
            0xFF01 ..= 0xFF02 => { 0xFF }, // serial transfer
            0xFF04 ..= 0xFF07 => { self.timer.read_byte(address) },
            0xFF0F => { self.interrupt_flag },
            0xFF10 ..= 0xFF3F => { self.sram[address as usize - 0xFF10] },
            0xFF40 ..= 0xFF4F => { self.ppu.read_byte(address) },
            0xFF4D => (if self.speed == Speed::FAST { 0x80 } else { 0 }) | (if self.switch_speed { 1 } else { 0 }),
            0xFF51 ..= 0xFF55 => { self.dma.borrow_mut().read_byte(address) },
            0xFF68 ..= 0xFF6B => { self.ppu.read_byte(address) },
            0xFF70 ..= 0xFF70 => { self.wram_bank as u8 },
            0xFF80 ..= 0xFFFE => { self.zram[(address - 0xFF80) as usize] },
            0xFFFF ..= 0xFFFF => { self.interrupt_enable },
            _ => 0,
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x0000 ..= 0x7FFF => { match &mut self.cartridge { Some(c) => c.write_byte(address, value), None => () } },
            0x8000 ..= 0x9FFF => { self.ppu.write_byte(address, value) },
            0xA000 ..= 0xBFFF => { match &mut self.cartridge { Some(c) => c.write_byte(address, value), None => () } },
            0xC000 ..= 0xCFFF | (0xE000 ..= 0xEFFF) => { self.wram[address as usize & 0x0FFF] = value },
            0xD000 ..= 0xDFFF | (0xF000 ..= 0xFDFF) => { self.wram[(self.wram_bank * 0x1000) | (address as usize & 0x0FFF)] = value },
            0xFE00 ..= 0xFE9F => { self.ppu.write_byte(address, value) },
            0xFF00 => { self.joypad.write_byte(address, value)},
            0xFF01 ..= 0xFF02 => { }, // serial transfer
            0xFF04 ..= 0xFF07 => { self.timer.write_byte(address, value) },
            0xFF10 ..= 0xFF3F => { self.sram[address as usize - 0xFF10] = value },
            0xFF40 ..= 0xFF4F => { self.ppu.write_byte(address, value) },
            0xFF46 => { execute_odma(self, value) },
            0xFF4D => { if value & 0x1 == 0x1 { self.switch_speed = true; } },
            0xFF51 ..= 0xFF55 => { self.dma.borrow_mut().write_byte(address, value)},
            0xFF68 ..= 0xFF6B => { self.ppu.write_byte(address, value)},
            0xFF0F => { self.interrupt_flag = value },
            0xFF70 ..= 0xFF70 => { self.wram_bank = match value & 0x7 { 0 => 1, n => n as usize }; },
            0xFF80 ..= 0xFFFE => { self.zram[(address - 0xFF80) as usize] = value; },
            0xFFFF ..= 0xFFFF => { self.interrupt_enable = value },
            _ => {},
        };
    }

    pub fn read_word(&self, address: u16) -> u16 {
        let low = (self.read_byte(address) as u16);
        let high  = (self.read_byte(address + 1) as u16);

        return (high << 16) | low;
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

    pub fn execute_ticks(&mut self, ticks: u32) {
        let cpu_divider = match self.speed {
            Speed::SLOW => 1,
            Speed::FAST => 2,
        };

        let mut dma = self.dma.clone();

        let vram_ticks = dma.borrow_mut().execute_tick(self);
        let gpu_ticks = ticks / cpu_divider + vram_ticks;
        let timer_ticks = ticks + vram_ticks * cpu_divider;

        self.timer.execute_ticks(timer_ticks);
        self.ppu.execute_ticks(gpu_ticks);

        // Gather interrupts

        self.interrupt_flag |= self.timer.interrupt;
        self.timer.interrupt = 0;

        self.interrupt_flag |= self.ppu.interrupt;
        self.ppu.interrupt = 0;
    }

    pub fn reset(&mut self) {
        let mut dma = self.dma.clone();

        self.timer.reset();
        self.dma.borrow_mut().reset();

        self.write_byte(0xFF05, 0);
        self.write_byte(0xFF06, 0);
        self.write_byte(0xFF07, 0);
        self.write_byte(0xFF10, 0x80);
        self.write_byte(0xFF11, 0xBF);
        self.write_byte(0xFF12, 0xF3);
        self.write_byte(0xFF14, 0xBF);
        self.write_byte(0xFF16, 0x3F);
        self.write_byte(0xFF16, 0x3F);
        self.write_byte(0xFF17, 0);
        self.write_byte(0xFF19, 0xBF);
        self.write_byte(0xFF1A, 0x7F);
        self.write_byte(0xFF1B, 0xFF);
        self.write_byte(0xFF1C, 0x9F);
        self.write_byte(0xFF1E, 0xFF);
        self.write_byte(0xFF20, 0xFF);
        self.write_byte(0xFF21, 0);
        self.write_byte(0xFF22, 0);
        self.write_byte(0xFF23, 0xBF);
        self.write_byte(0xFF24, 0x77);
        self.write_byte(0xFF25, 0xF3);
        self.write_byte(0xFF26, 0xF1);
        self.write_byte(0xFF40, 0x91);
        self.write_byte(0xFF42, 0);
        self.write_byte(0xFF43, 0);
        self.write_byte(0xFF45, 0);
        self.write_byte(0xFF47, 0xFC);
        self.write_byte(0xFF48, 0xFF);
        self.write_byte(0xFF49, 0xFF);
        self.write_byte(0xFF4A, 0);
        self.write_byte(0xFF4B, 0);
    }

    //https://github.com/rustwasm/wasm-bindgen/issues/1052
    //https://stackoverflow.com/questions/52796222/how-to-pass-an-array-of-objects-to-webassembly-and-convert-it-to-a-vector-of-str
    pub fn load_cartridge(&mut self, result: &JsValue) {
        let bytes: Vec<u8> = result.into_serde().unwrap();
        let cartridge_type = bytes[HEADER_INDEX_FOR_CARTRIDGE_TYPE];
        let mut cartridge = Cartridge::new(bytes);
        match cartridge_type {
            0x0 => { cartridge.cartridge_type = Some(CartridgeType::MBC0) },
            0x01..=0x03 => { cartridge.cartridge_type = Some(CartridgeType::MBC1) },
            //0x05..=0x06 => { cartridge.cartridge_type = Some(CartridgeType::MBC2) },
            //0x0F..=0x13 => { cartridge.cartridge_type = Some(CartridgeType::MBC3) },
            0x1E => { cartridge.cartridge_type = Some(CartridgeType::MBC5) },
            _ => panic!("cartridge type not implemented")
        }
        self.cartridge = Some(cartridge);
    }
}
