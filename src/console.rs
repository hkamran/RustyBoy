use crate::cpu::Cpu;
use crate::mmu::Mmu;
use web_sys::CanvasRenderingContext2d;

#[derive(PartialEq, Copy, Clone)]
pub enum GameboyType {
use wasm_bindgen::prelude::*;

pub enum Mode {
    COLOR,
    CLASSIC
}

#[allow(dead_code)]
pub struct Console {
    pub cpu: Cpu,
    pub mmu: Mmu,
}

impl Console {

    pub fn new() -> Self {
        return Console {
            mmu: Mmu::new(),
            cpu: Cpu::new()
        }
    }

    pub fn load(&mut self, cart_path: &str) {
        self.mmu.load_cartridge(cart_path);
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
        self.mmu.reset();
    }

    pub fn execute_ticks(&mut self, ticks: u32) -> () {
        for _i in 0 .. ticks {
            self.execute_tick();
        }
    }

    pub fn execute_tick(&mut self) {
        let cpu_ticks = self.cpu.execute_tick(&mut self.mmu) * 4;
        self.mmu.execute_ticks(cpu_ticks);
    }

}
