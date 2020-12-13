use crate::mmu::Mmu;

#[derive(PartialEq)]
enum DMAType {
    NONE,
    GDMA,
    HDMA,
}

pub struct Dma {
    dma: [u8; 4],
    dma_status: DMAType,
    dma_source: u16,
    dma_destination: u16,
    dma_length: u8,
}

impl Dma {

    pub fn new() -> Self {
        return Dma {
            dma: [0; 4],
            dma_status: DMAType::NONE,
            dma_source: 0,
            dma_destination: 0,
            dma_length: 0,
        }
    }

    pub fn reset(&mut self) {
        self.dma_source = 0;
        self.dma_destination = 0;
        self.dma_length = 0;
        self.dma_status = DMAType::NONE;
    }

    pub fn execute_tick(&mut self, mmu: &mut Mmu) -> u32 {
        return match self.dma_status {
            DMAType::NONE => 0,
            DMAType::GDMA => self.execute_gdma(mmu),
            DMAType::HDMA => self.execute_hdma(mmu),
        };
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0xFF51 ..= 0xFF54 => { self.dma[(address - 0xFF51) as usize] },
            0xFF55 => self.dma_length | if self.dma_status == DMAType::NONE { 0x80 } else { 0 },
            _ => panic!("{:04X}", address),
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0xFF51 => self.dma[0] = value,
            0xFF52 => self.dma[1] = value & 0xF0,
            0xFF53 => self.dma[2] = value & 0x1F,
            0xFF54 => self.dma[3] = value & 0xF0,
            0xFF55 => {
                if self.dma_status == DMAType::HDMA {
                    if value & 0x80 == 0 { self.dma_status = DMAType::NONE; };
                    return;
                }
                let src = ((self.dma[0] as u16) << 8) | (self.dma[1] as u16);
                let dst = ((self.dma[2] as u16) << 8) | (self.dma[3] as u16) | 0x8000;
                if !(src <= 0x7FF0 || (src >= 0xA000 && src <= 0xDFF0)) { panic!("HDMA transfer with illegal start address {:04X}", src); }

                self.dma_source = src;
                self.dma_destination = dst;
                self.dma_length = value & 0x7F;

                self.dma_status =
                    if value & 0x80 == 0x80 { DMAType::HDMA }
                    else { DMAType::GDMA };
            },
            _ => panic!("The address {:04X} should not be handled by hdma_write", address),
        };
    }

    fn execute_hdma(&mut self, mmu: &mut Mmu) -> u32 {
        if mmu.ppu.h_blank == false {
            return 0;
        }
        self.execute_transfer(mmu);
        if self.dma_length == 0x7F { self.dma_status = DMAType::NONE; }

        return 8;
    }

    fn execute_gdma(&mut self, mmu: &mut Mmu) -> u32 {
        let len = self.dma_length as u32 + 1;
        for _i in 0 .. len {
            self.execute_transfer(mmu);
        }

        self.dma_status = DMAType::NONE;
        return len * 8;
    }

    fn execute_transfer(&mut self, mmu: &mut Mmu) {
        let mmu_src = self.dma_source;
        for j in 0 .. 0x10 {
            let b: u8 = mmu.read_byte(mmu_src + j);
            mmu.ppu.write_byte(self.dma_destination + j, b);
        }
        self.dma_source += 0x10;
        self.dma_destination += 0x10;

        if self.dma_length == 0 {
            self.dma_length = 0x7F;
        }
        else {
            self.dma_length -= 1;
        }
    }

}

// OAM DMA
pub fn execute_odma(mmu: &mut Mmu, value: u8) {
    let base = (value as u16) << 8;
    for i in 0 .. 0xA0 {
        let data = mmu.read_byte(base + i);
        mmu.write_byte(0xFE00 + i, data);
    }
}
