use crate::mmu::Mmu;
use crate::operations::execute_operation;

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
    pub interrupt_master_enable: bool,
    pub disable_interrupt_counter: u8, // Schedules interrupt handling to be enabled after the next machine cycle
    pub enable_interrupt_counter: u8,  // Schedules interrupt handling to be enabled after the next machine cycle

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
            interrupt_master_enable: true,
            disable_interrupt_counter: 0,
            enable_interrupt_counter: 0,
            cycles: 0,
        }
    }

    pub fn tick(&mut self, mmu: &mut Mmu) -> u8 {
        let cycles = self.cycles;
        let pc = self.pc;

        self.update_interrupt_master_flag();
        if self.handle_interrupt(mmu) {
            return (self.cycles - cycles) as u8;
        }

        if self.halted {
            return 1;
        }

        let opcode: u8 = mmu.read_byte(pc);
        execute_operation(opcode, self, mmu);

        return (self.cycles - cycles) as u8;
    }

    pub fn update_interrupt_master_flag(&mut self) {
        // There are some operations where interrupts are triggered after the next machine cycle
        // So to mimic it we will use a counter that will tick down.

        self.disable_interrupt_counter = match self.disable_interrupt_counter {
            2 => 1,
            1 => { self.interrupt_master_enable = false; 0},
            _ => 0,
        };
        self.enable_interrupt_counter = match self.enable_interrupt_counter {
            2 => 1,
            1 => { self.interrupt_master_enable = true; 0},
            _ => 0,
        };
    }

    pub fn handle_interrupt(&mut self, mmu: &mut Mmu) -> bool {
        if self.interrupt_master_enable == false &&
            self.halted == false {
            return false;
        }

        // http://bgb.bircd.org/pandocs.htm#interrupts
        let interrupt_type = mmu.interrupt_enable & mmu.interrupt_flag;
        if interrupt_type == 0 { return false }

        self.halted = false;
        if self.interrupt_master_enable == false { return false; };
        self.interrupt_master_enable = false;

        // http://bgb.bircd.org/pandocs.htm#interrupts
        let bit = interrupt_type.trailing_zeros();
        if bit >= 5 { panic!("Invalid interrupt code") }

        // clear flag
        mmu.interrupt_flag &= !(1 << bit);

        let pc = self.pc;
        self.push_word(mmu, pc);

        // go to the vector
        self.pc = 0x0040 | ((bit as u16) << 3);

        return true;
    }

    pub fn to_string(&mut self) -> String {
        return format!("PC: {:#06X}, ", self.pc);
    }

    pub fn print(&mut self) {
        println!("{}", self.to_string());
    }

    // register helpers

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

    // operation helpers

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

    pub fn apply_rotate_left_with_flags(&mut self, arg: u8, apply_with_carry: bool) -> u8 {
        let carry: u8 = if arg & 0x80 == 0x80 {1} else {0};
        let bit = if apply_with_carry { if self.get_f_carry() {0x01} else {0} } else { if carry == 1 {0x01} else {0} };
        let result: u8 = (arg << 1) | bit;

        self.set_f_half_carry(false);
        self.set_f_negative(false);
        self.set_f_zero(result == 0);
        self.set_f_carry(carry > 0);

        return result;
    }

    pub fn apply_rotate_right_with_flags(&mut self, arg: u8, apply_with_carry: bool) -> u8 {
        let carry: u8 = arg & 0x1;
        let bit = if apply_with_carry { if self.get_f_carry() {0x80} else {0} } else { if carry == 1 {0x80} else {0} };
        let result = (arg >> 1) | bit;

        self.set_f_half_carry(false);
        self.set_f_negative(false);
        self.set_f_zero(result == 0);
        self.set_f_carry(carry > 0);

        return result;
    }

    pub fn apply_add8_with_flags(&mut self, a: u8, b: u8, apply_with_carry: bool) -> u8 {
        let carry: u8 = if apply_with_carry { if self.get_f_carry() {1} else {0} } else { 0 };
        let result = a.wrapping_add(b).wrapping_add(carry);

        self.set_f_zero(result == 0);
        self.set_f_half_carry(((a & 0xF) + (b & 0xF) + carry) > 0xF);
        self.set_f_negative(false);
        self.set_f_carry((a as u16) + (b as u16) + (carry as u16) > 0xFF);

        return result;
    }

    pub fn apply_sub8_with_flags(&mut self, a: u8, b: u8, apply_with_carry: bool) -> u8 {
        let carry: u8 = if apply_with_carry { if self.get_f_carry() {1} else {0} } else { 0 };
        let result = a.wrapping_sub(b).wrapping_sub(carry);

        self.set_f_zero(result == 0);
        self.set_f_half_carry((a & 0x0F) < (b & 0x0F) + carry);
        self.set_f_negative(true);
        self.set_f_carry((a as u16) < (b as u16) + (carry as u16));

        return result;
    }

    pub fn apply_add16_with_flags(&mut self, a: u16, b: u16) -> u16 {
        let result: u16 = a.wrapping_add(b);

        self.set_f_half_carry((a & 0x07FF) + (b & 0x07FF) > 0x07FF);
        self.set_f_negative(false);
        self.set_f_carry(a > 0xFFFF - b);

        return result;
    }

    pub fn apply_and8_with_flags(&mut self, a: u8, b: u8) -> u8 {
        let result = a & b;

        self.set_f_zero(result == 0);
        self.set_f_half_carry(true);
        self.set_f_carry(false);
        self.set_f_negative(false);

        return result;
    }

    pub fn apply_or8_with_flags(&mut self, a: u8, b: u8) -> u8 {
        let result = a | b;

        self.set_f_zero(result == 0);
        self.set_f_half_carry(false);
        self.set_f_carry(false);
        self.set_f_negative(false);

        return result;
    }

    pub fn apply_xor8_with_flags(&mut self, a: u8, b: u8) -> u8 {
        let result = a ^ b;

        self.set_f_zero(result == 0);
        self.set_f_half_carry(false);
        self.set_f_carry(false);
        self.set_f_negative(false);

        return result;
    }

    pub fn apply_shift_left_with_flags(&mut self, a: u8) -> u8 {
        let carry = if a & 0x80 == 0x80 {1} else {0};
        let result = a << 1;

        self.set_f_half_carry(false);
        self.set_f_negative(false);
        self.set_f_zero(result == 0);
        self.set_f_carry(carry == 1);

        return result;
    }

    pub fn apply_shift_right_with_flags(&mut self, a: u8, apply_rotation: bool) -> u8 {
        let carry = if a & 0x01 == 0x01 {1} else {0};
        let result = (a >> 1) | if apply_rotation {(a & 0x80)} else {0};

        self.set_f_half_carry(false);
        self.set_f_negative(false);
        self.set_f_zero(result == 0);
        self.set_f_carry(carry == 1);

        return result;
    }

    pub fn apply_swap_bytes(&mut self, a: u8) -> u8 {
        let result = (a >> 4) | (a << 4);

        self.set_f_zero(a == 0);
        self.set_f_half_carry(false);
        self.set_f_negative(false);
        self.set_f_carry(false);

        return result;
    }

    pub fn apply_bit_test(&mut self, value: u8, bit: u8) {
        self.set_f_negative(false);
        self.set_f_half_carry(true);
        self.set_f_zero(value & (1 << (bit as u32)) == 0);
    }

    pub fn push_byte(&mut self, mmu: &mut Mmu, value: u8) {
        self.sp -= 1;
        mmu.write_byte(self.sp, value);
    }

    pub fn push_word(&mut self, mmu: &mut Mmu, value: u16) {
        self.sp -= 2;
        mmu.write_word(self.sp, value);
    }

    pub fn pop_byte(&mut self, mmu: &mut Mmu) -> u8 {
        let value = mmu.read_byte(self.sp);
        self.sp -= 1;
        return value;
    }

    pub fn pop_word(&mut self, mmu: &mut Mmu) -> u16 {
        let value = mmu.read_word(self.sp);
        self.sp -= 2;
        return value;
    }
}

