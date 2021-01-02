use crate::cartridge::{Cartridge, CartridgeType, HEADER_INDEX_FOR_CARTRIDGE_TYPE};
use crate::ppu::Ppu;
use crate::psg::Psg;
use crate::dma::{Dma, execute_dma_tick, execute_odma};
use crate::timer::Timer;
use std::cell::RefCell;
use std::rc::Rc;
use crate::joypad::Joypad;
use crate::console::GameboyType;
use wasm_bindgen::prelude::*;
use std::path::Path;
use std::fs;

#[wasm_bindgen]
pub struct Mmu {
    hram: [u8; 0x7F],
    wram: [u8; 0x8000],
    wram_bank: usize,
    switch_speed: bool,
    pub speed: Speed,
    pub interrupt_enable: u8,
    pub interrupt_flags: u8,
    psg: Psg,
    #[wasm_bindgen(skip)]
    pub ppu: Ppu,
    cartridge: Cartridge,
    pub dma: Dma,
    pub timer: Timer,
    pub joypad: Joypad,
    pub model: GameboyType,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Speed {
    FAST, SLOW
}

#[allow(unused)]
impl Mmu {

    pub fn new() -> Self {
        return Mmu {
            psg: Psg::new(),
            wram: [0; 0x8000],
            wram_bank: 1,
            hram: [0; 0x7F],
            speed: Speed::SLOW,
            switch_speed: false,
            interrupt_flags: 0,
            interrupt_enable: 0,

            cartridge: Cartridge::new(),
            ppu: Ppu::new(),
            dma: Dma::new(),
            timer: Timer::new(),
            joypad: Joypad::new(),
            model: GameboyType::CLASSIC
        };
    }

    pub fn load_cartridge_from_js_value(&mut self, result: &JsValue) {
        let bytes: Vec<u8> = result.into_serde().unwrap();
        self.load_cartridge_from_bytes(bytes);
    }

    pub fn load_from_file_address(&mut self, file_path: &str) {
        let path = Path::new(file_path);
        let bytes : Vec<u8> = fs::read(path).expect("yabe");
        self.load_cartridge_from_bytes(bytes);
    }

    pub fn load_cartridge_from_bytes(&mut self, bytes: Vec<u8>) {
        let cartridge_type = bytes[HEADER_INDEX_FOR_CARTRIDGE_TYPE];
        match cartridge_type {
            0x00 ..= 0x00 => { self.cartridge.cartridge_type = CartridgeType::MBC0 },
            0x01 ..= 0x03 => { self.cartridge.cartridge_type = CartridgeType::MBC1 },
            //0x05..=0x06 => { cartridge.cartridge_type = Some(CartridgeType::MBC2) },
            0x0F ..= 0x13 => { self.cartridge.cartridge_type = CartridgeType::MBC3 },
            0x19 ..= 0x1E => { self.cartridge.cartridge_type = CartridgeType::MBC5 },
            _ => panic!("cartridge type not implemented")
        }
        self.cartridge.set_rom(bytes);
        self.model = self.cartridge.get_gameboy_type().clone();
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000 ..= 0x7FFF => { self.cartridge.read_byte(address) },
            0x8000 ..= 0x9FFF => { self.ppu.read_byte(address) },
            0xA000 ..= 0xBFFF => { self.cartridge.read_byte(address) },
            0xC000 ..= 0xCFFF | (0xE000 ..= 0xEFFF) => { self.wram[address as usize & 0x0FFF] },
            0xD000 ..= 0xDFFF | (0xF000 ..= 0xFDFF) => { self.wram[(self.wram_bank * 0x1000) | address as usize & 0x0FFF] },
            0xFE00 ..= 0xFE9F => { self.ppu.read_byte(address) },
            0xFF00 ..= 0xFF00 => { self.joypad.read_byte(address) },
            0xFF01 ..= 0xFF02 => { 0xFF }, // serial transfer
            0xFF04 ..= 0xFF07 => { self.timer.read_byte(address) },
            0xFF0F => { self.interrupt_flags },
            0xFF10 ..= 0xFF3F => { self.psg.read_byte(address) },
            0xFF4D => (if self.speed == Speed::FAST { 0x80 } else { 0 }) | (if self.switch_speed { 1 } else { 0 }),
            0xFF40 ..= 0xFF4F => { self.ppu.read_byte(address) },
            0xFF51 ..= 0xFF55 => { self.dma.read_byte(address) },
            0xFF68 ..= 0xFF6C => { self.ppu.read_byte(address) },
            0xFF70 ..= 0xFF70 => { self.wram_bank as u8 },
            0xFF80 ..= 0xFFFE => { self.hram[address as usize & 0x007F] },
            0xFFFF => { self.interrupt_enable },
            _ => 0,
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x0000 ..= 0x7FFF => { self.cartridge.write_byte(address, value) },
            0x8000 ..= 0x9FFF => { self.ppu.write_byte(address, value) },
            0xA000 ..= 0xBFFF => { self.cartridge.write_byte(address, value) },
            0xC000 ..= 0xCFFF | (0xE000 ..= 0xEFFF) => { self.wram[address as usize & 0x0FFF] = value },
            0xD000 ..= 0xDFFF | (0xF000 ..= 0xFDFF) => { self.wram[(self.wram_bank * 0x1000) | (address as usize & 0x0FFF)] = value },
            0xFE00 ..= 0xFE9F => { self.ppu.write_byte(address, value) },
            0xFF00 => { self.joypad.write_byte(address, value) },
            0xFF01 ..= 0xFF02 => { }, // serial transfer
            0xFF04 ..= 0xFF07 => { self.timer.write_byte(address, value) },
            0xFF0F => { self.interrupt_flags = value },
            0xFF10 ..= 0xFF3F => { self.psg.write_byte(address, value) },
            0xFF46 => { execute_odma(self, value) },
            0xFF4D => { if value & 0x1 == 0x1 { self.switch_speed = true; } },
            0xFF40 ..= 0xFF4F => { self.ppu.write_byte(address, value) },
            0xFF51 ..= 0xFF55 => { self.dma.write_byte(address, value) },
            0xFF68 ..= 0xFF6B => { self.ppu.write_byte(address, value) },
            0xFF70 ..= 0xFF70 => { self.wram_bank = match value & 0x7 { 0 => 1, n => n as usize }; },
            0xFF80 ..= 0xFFFE => { self.hram[address as usize & 0x007F] = value; },
            0xFFFF => { self.interrupt_enable = value },
            _ => {},
        };
    }

    pub fn read_word(&self, address: u16) -> u16 {
        let low = (self.read_byte(address) as u16);
        let high  = (self.read_byte(address + 1) as u16);

        return high.overflowing_shl(8).0 | low;
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

        let dma_ticks = execute_dma_tick(self);
        let gpu_ticks = ticks / cpu_divider + dma_ticks;
        let timer_ticks = ticks + dma_ticks * cpu_divider;

        self.timer.execute_ticks(timer_ticks);
        self.ppu.execute_ticks(gpu_ticks);

        // Gather interrupts

        self.interrupt_flags |= self.timer.interrupt_flags;
        self.timer.interrupt_flags = 0;

        self.interrupt_flags |= self.ppu.interrupt_flags;
        self.ppu.interrupt_flags = 0;
    }

    pub fn reset(&mut self, model: GameboyType) {
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

}
