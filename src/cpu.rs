use crate::mmu::Mmu;
use crate::operations;

#[allow(unused)]
pub struct Cpu {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,

    pub halted: bool,
    pub ime: bool,
    pub cycles: u16,
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
        }
    }

    pub fn tick(&mut self, mmu: &mut Mmu) -> u8 {
        let cycles = self.cycles;
        let pc = self.pc;
        let opcode: u8 = mmu.read_byte(pc);
        let operation = operations::get(opcode);

        // execute
        operation(self, mmu);

        return (self.cycles - cycles) as u8;
    }

    pub fn to_string(&mut self) -> String {
        return format!("PC: {:#06X}, ", self.pc);
    }

    pub fn print(&mut self) {
        println!("{}", self.to_string());
    }

    // Register functions

    pub fn get_af(&self) -> u16 {
        return (self.a as u16) << 8
            | self.f as u16;
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = (value & 0xFF) as u8;
    }

    pub fn get_bc(&self) -> u16 {
        return (self.b as u16) << 8
            | self.c as u16;
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    pub fn get_de(&self) -> u16 {
        return (self.d as u16) << 8
            | self.e as u16;
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    pub fn get_hl(&self) -> u16 {
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

    pub fn set_f_negative(&mut self, value: bool) {
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

    pub fn get_f_carry(&self) -> bool {
        return self.f & 0x10 > 0;
    }

    pub fn get_f_substract(&self) -> bool {
        return self.f & 0x40 > 0;
    }

    pub fn get_f_half_carry(&self) -> bool {
        return self.f & 0x20 > 0;
    }

    pub fn get_f_zero(&self) -> bool {
        return self.f & 0x80 > 0;
    }

    pub fn apply_inc8_with_flags(&mut self, arg: u8) -> u8 {
        let value = arg.wrapping_add(1);

        self.set_f_zero(value == 0);
        self.set_f_half_carry((arg & 0x0F) + 1 > 0x0F);
        self.set_f_negative(false);

        return value;
    }

    pub fn apply_dec8_with_flags(&mut self, arg: u8) -> u8 {
        let value = arg.wrapping_sub(1);

        self.set_f_zero(value == 0);
        self.set_f_half_carry((arg & 0x0F) == 0x0F);
        self.set_f_negative(true);

        return value;
    }

    pub fn apply_rotate_left_with_flags(&mut self, arg: u8) -> u8 {
        let carry: u8 = if arg & 0x80 != 0 {1} else {0};
        let result: u8 = (self.a << 1) | carry;

        self.set_f_half_carry(false);
        self.set_f_negative(false);
        self.set_f_zero(result == 0);
        self.set_f_carry(carry > 0);

        return result;
    }

    pub fn apply_add16_with_flags(&mut self, a: u16, b: u16) -> u16 {
        let result: u16 = a.wrapping_add(b);

        self.set_f_half_carry((a & 0x07FF) + (b & 0x07FF) > 0x07FF);
        self.set_f_negative(false);
        self.set_f_carry(a > 0xFFFF - b);

        return result;
    }

}

