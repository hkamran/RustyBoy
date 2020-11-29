use crate::cpu::operations::OperationType;

use std::collections::HashMap;
use crate::mmu::Mmu;

#[allow(unused)]
pub struct Cpu {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,

    halted: bool,
    ime: bool,
    cycles: u16,

    operations: HashMap<u8, OperationType>,
}

#[allow(unused)]
impl Cpu {

    pub fn new() -> Self {
        return Cpu {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
            halted: false,
            ime: false,
            cycles: 0,
            operations: operations::create(),
        }
    }

    pub fn tick(&mut self, bus: &mut Mmu) -> u8 {
        let cycles = self.cycles;
        let pc = self.pc;
        let opcode: u8 = 0;

        let operations = (*self.operations);
        let operation = operations.get(&opcode).unwrap();

        // execute
        operation(self, bus);

        return (self.cycles - cycles) as u8;
    }

    pub fn to_string(&mut self) -> String {
        return format!("PC: {:#06X}, ", self.pc);
    }

    pub fn print(&mut self) {
        println!("{}", self.to_string());
    }

    // Register functions

    pub fn get_af(&mut self) -> u16 {
        return (self.a as u16) << 8
            | self.f as u16;
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = (value & 0xFF) as u8;
    }

    pub fn get_bc(&mut self) -> u16 {
        return (self.b as u16) << 8
            | self.c as u16;
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    pub fn get_de(&mut self) -> u16 {
        return (self.d as u16) << 8
            | self.e as u16;
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    pub fn get_hl(&mut self) -> u16 {
        return (self.h as u16) << 8
            | self.l as u16;
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }

    pub fn set_f_carry(&mut self, value: bool) {
        if value == true {
            self.f |= 0x10;
        } else {
            self.f &= 0xEF;
        }
    }

    pub fn set_f_substract(&mut self, value: bool) {
        if value == true {
            self.f |= 0x40;
        } else {
            self.f &= 0xBF;
        }
    }

    pub fn set_f_half_carry(&mut self, value: bool) {
        if value == true {
            self.f |= 0x20;
        } else {
            self.f &= 0xDF;
        }
    }

    pub fn set_f_zero(&mut self, value: bool) {
        if value == true {
            self.f |= 0x80;
        } else {
            self.f &= 0x7F;
        }
    }

    pub fn get_f_carry(&mut self) -> bool {
        return self.f & 0x10 > 0;
    }

    pub fn get_f_substract(&mut self) -> bool {
        return self.f & 0x40 > 0;
    }

    pub fn get_f_half_carry(&mut self) -> bool {
        return self.f & 0x20 > 0;
    }

    pub fn get_f_zero(&mut self) -> bool {
        return self.f & 0x80 > 0;
    }

    pub fn inc8_with_flags(&mut self, arg: u8) -> u8 {
        let value = arg.wrapping_add(1);

        self.set_f_zero(value == 0);
        self.set_f_half_carry((arg & 0x0F) + 1 > 0x0F);
        self.set_f_substract(false);

        return value;
    }

    pub fn dec8_with_flags(&mut self, arg: u8) -> u8 {
        let value = arg.wrapping_sub(1);

        self.set_f_zero(value == 0);
        self.set_f_half_carry((arg & 0x0F) == 0x0F);
        self.set_f_substract(true);

        return value;
    }

}

pub mod operations {
    use std::collections::HashMap;
    use crate::mmu::Mmu;
    use crate::cpu::Cpu;

    pub type OperationType = fn(cpu: &mut Cpu, bus: &mut Mmu) -> ();

    pub fn create() -> HashMap<u8, OperationType> {
        let mut map: HashMap<u8, OperationType> = HashMap::new();

        for x in 0u8..=0xFF {
            map.insert(x, opcode_00_nop);
        }

        return map;
    }

    pub fn op_00_nop(cpu: &mut Cpu, mmu: &mut Mmu) {
        cpu.pc += 1;
        cpu.cycles += 2;
    }

    pub fn op_01_lda_bc_xx(cpu: &mut Cpu, mmu: &mut Mmu) {
        cpu.set_bc(mmu.read_word(cpu.pc));
        cpu.pc += 2;
        cpu.cycles += 3;
    }

    pub fn op_02_lda_bc_a(cpu: &mut Cpu, mmu: &mut Mmu) {
        mmu.write_byte(cpu.get_bc(), cpu.a);
        cpu.pc += 1;
        cpu.cycles += 7
    }

    pub fn op_03_inc_bc(cpu: &mut Cpu, mmu: &mut Mmu) {
        cpu.set_bc(cpu.get_bc().wrapping_add(1));
        cpu.pc += 1;
        cpu.cyles += 6;
    }

    pub fn op_04_inc_b(cpu: &mut Cpu, mmu: &mut Mmu) {
        cpu.b = cpu.inc8_with_flags(cpu.b);
        cpu.pc += 1;
        cpu.cyles += 4;
    }

    pub fn op_05_dec_b(cpu: &mut Cpu, mmu: &mut Mmu) {
        cpu.b = cpu.dec8_with_flags(cpu.b);
        cpu.pc += 1;
        cpu.cyles += 4;
    }

}