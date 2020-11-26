use crate::bus::Bus;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct Registers {
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
}

impl Registers {

    pub fn new() -> Self {
        return Registers {
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
        }
    }

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
}

#[allow(unused)]
pub struct Cpu<'a> {
    pub registers: Box<Registers>,
    pub bus: &'a Bus<'a>,
    pub operations: Operations<'a>
}

#[allow(unused)]
impl<'a> Cpu<'a> {

    pub fn new(bus: &'a Bus) -> Self {
        let registers = Box::new(Registers::new());
        return Cpu {
            registers,
            bus,
            operations: Operations::new(registers.as_ref(), bus)
        }
    }

    pub fn tick(&mut self) -> () {
        let mut pc = self.registers.pc;
        let opcode = self.bus.read_byte(pc);

        self.operations.execute(opcode);

        let offset = self.operations.offset;
        let cycles = self.operations.cycles;
    }

}

#[allow(dead_code)]
pub struct Operations<'a> {
    pub cycles: u8,
    pub offset: u8,

    pub registers: &'a Registers,
    pub bus: &'a Bus<'a>,
    pub codes: HashMap<u8, fn() -> ()>,
}

#[allow(unused)]
impl<'a> Operations<'a> {

    pub fn new(registers: &'a Registers, bus: &'a Bus) -> Self {
        let mut codes: HashMap<u8, fn() -> ()> = HashMap::new();

        for code in 0..0xFF {
        }

        return Operations {
            cycles: 0,
            offset: 0,
            registers,
            bus,
            codes,
        }
    }

    pub fn execute(&mut self, opcode: u8) {
        let op: Option<&mut fn()> = self.codes.get_mut(&opcode);
        if op.is_none() {
            panic!(format!("Unknown opcode given: {}", opcode))
        }
        op.unwrap()();
    }

    fn get_operand(&mut self, index: u8) -> u8 {
        let pc = self.registers.pc;
        let operand = self.bus.read_byte(pc + index as u16);

        return operand;
    }

    fn nop(&mut self) {
        let operand = self.get_operand(1);

        self.cycles = 2;
        self.offset = 1;
    }

}