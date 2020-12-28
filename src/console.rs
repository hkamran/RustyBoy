use crate::cpu::Cpu;
use crate::mmu::Mmu;
use web_sys::CanvasRenderingContext2d;
use wasm_bindgen::prelude::*;
use console_error_panic_hook;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameboyType {
    COLOR,
    CLASSIC
}

#[wasm_bindgen]
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Console {
    pub cpu: Cpu,
    pub mmu: Mmu,
}

#[wasm_bindgen]
impl Console {

    pub fn new() -> Self {
        console_error_panic_hook::set_once();

        return Console {
            mmu: Mmu::new(),
            cpu: Cpu::new()
        }
    }

    pub fn load(&mut self, result: &JsValue) {
        self.mmu.load_cartridge(result);
    }

    pub fn reset(&mut self) {
        let model = self.mmu.model;
        self.cpu.reset(model.clone());
        self.mmu.reset(model.clone());
        self.mmu.timer.reset(model.clone());
        self.mmu.ppu.reset(model.clone());
        self.mmu.dma.reset(model);
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
