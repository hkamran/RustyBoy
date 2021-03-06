use crate::mmu::Mmu;
use crate::operations::execute_operation;
use crate::logger::log;
use crate::console::GameboyType;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[allow(unused)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

    pub cycles: u32,
    pub ticks: u32,
    pub opcode: u16,
}

#[allow(unused)]
impl Cpu {

    pub fn new() -> Self {
        return Cpu {
            a: 0x00,
            b: 0x00,
            c: 0x00,
            d: 0x00,
            e: 0x00,
            f: 0x00,
            h: 0x00,
            l: 0x00,
            pc: 0x0000,
            sp: 0x0000,
            halted: false,
            interrupt_master_enable: true,
            disable_interrupt_counter: 0,
            enable_interrupt_counter: 0,
            cycles: 0,
            opcode: 0,
            ticks: 0,
        }
    }

    pub fn reset(&mut self, model: GameboyType) {
        self.a = 0x11;
        self.b = 0x00;
        self.c = 0x13;
        self.d = 0x00;
        self.e = 0xD8;
        self.f = 0xB0;
        self.h = 0x01;
        self.l = 0x4D;
        self.pc = 0x0100;
        self.sp = 0xFFFE;
        self.halted = false;
        self.interrupt_master_enable = true;
        self.disable_interrupt_counter = 0;
        self.enable_interrupt_counter = 0;
        self.cycles = 0;
        self.opcode = 0;
        self.ticks = 0;

        if model == GameboyType::CLASSIC {
            self.a = 0x01;
        }
    }

    pub fn execute_ticks(&mut self, mmu: &mut Mmu, ticks: u32) -> u32 {
        let mut total = 0;
        for i in 0 .. ticks {
            total += self.execute_tick(mmu);
        }
        return total;
    }

    pub fn execute_tick(&mut self, mmu: &mut Mmu) -> u32 {
        let cycles = self.cycles;
        let pc = self.pc;

        self.update_interrupt_master_flag();
        if self.handle_interrupt(mmu) {
            return (self.cycles - cycles) as u32;
        }

        if self.halted {
            return 1;
        }

        self.opcode = mmu.read_byte(pc) as u8 as u16;
        //log(self.to_string());
        execute_operation(self.opcode as u8, self, mmu);

        self.ticks += 1;
        return (self.cycles - cycles) as u32;
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
        let interrupt_mask = mmu.interrupt_enable & mmu.interrupt_flags;
        if interrupt_mask == 0 { return false }

        self.halted = false;
        if self.interrupt_master_enable == false { return false; };
        self.interrupt_master_enable = false;

        // http://bgb.bircd.org/pandocs.htm#interrupts
        // Bit 0: V-Blank  Interrupt Enable  (INT 40h)  (1=Enable)
        // Bit 1: LCD STAT Interrupt Enable  (INT 48h)  (1=Enable)
        // Bit 2: Timer    Interrupt Enable  (INT 50h)  (1=Enable)
        // Bit 3: Serial   Interrupt Enable  (INT 58h)  (1=Enable)
        // Bit 4: Joypad   Interrupt Enable  (INT 60h)  (1=Enable)

        let interrupt_type = interrupt_mask.trailing_zeros();
        if interrupt_type >= 5 { panic!("Invalid interrupt code") }

        // clear flag
        mmu.interrupt_flags &= !(1 << interrupt_type);

        self.push_word(mmu, self.pc);

        // go to the vector
        self.pc = match interrupt_type {
            0 => 0x40,
            1 => 0x48,
            2 => 0x50,
            3 => 0x58,
            4 => 0x60,
            _ => { panic!("error") }
        };

        self.cycles += 12;

        return true;
    }

    pub fn to_string(&mut self) -> String {
        return format!("PC: {:#06X} OPCODE: {:#04X} A: {:#04X} B: {:#04X} C: {:#04X} D: {:#04X} E: {:#04X} F: {:#04X} H: {:#04X} L: {:#04X} SP: {:#06X}",
                       self.pc,
                       self.opcode,
                       self.a,
                       self.b,
                       self.c,
                       self.d,
                       self.e,
                       self.f,
                       self.h,
                       self.l,
                       self.sp
        );
    }

    pub fn print(&mut self) {
        println!("{}", self.to_string());
    }

    // register helpers

    pub fn get_af(&self) -> u16 {
        return (self.a as u16) << 8
            | (self.f & 0xF0) as u16;
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = (value & 0x00F0) as u8;
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
            self.f &= !0x10;
        }
    }

    pub fn set_f_negative(&mut self, value: bool) {
        if value == true {
            self.f |= 0x40;
        } else {
            self.f &= !0x40;
        }
    }

    pub fn set_f_half_carry(&mut self, value: bool) {
        if value == true {
            self.f |= 0x20;
        } else {
            self.f &= !0x20;
        }
    }

    pub fn set_f_zero(&mut self, value: bool) {
        if value == true {
            self.f |= 0x80;
        } else {
            self.f &= !0x80;
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

    pub fn apply_inc_u8_with_flags(&mut self, arg: u8) -> u8 {
        let value = arg.wrapping_add(1);

        self.set_f_zero(value == 0);
        self.set_f_half_carry((arg & 0x0F) + 1 > 0x0F);
        self.set_f_negative(false);

        return value;
    }

    pub fn apply_dec_u8_with_flags(&mut self, arg: u8) -> u8 {
        let value = arg.wrapping_sub(1);

        self.set_f_zero(value == 0);
        self.set_f_half_carry(((arg & 0xF) as i16) - 1 < 0);
        self.set_f_negative(true);

        return value;
    }

    pub fn apply_rotate_left_with_flags(&mut self, arg: u8, apply_with_carry: bool) -> u8 {
        let carry: u8 = if arg & 0x80 == 0x80 {1} else {0};
        let bit = if apply_with_carry { carry } else { if self.get_f_carry() {0x01} else {0} };
        let result: u8 = (arg << 1) | bit;

        self.set_f_half_carry(false);
        self.set_f_negative(false);
        self.set_f_zero(result == 0);
        self.set_f_carry(carry > 0);

        return result;
    }

    pub fn apply_rotate_right_with_flags(&mut self, arg: u8, apply_with_carry: bool) -> u8 {
        let carry: u8 = if arg & 0x1 == 0x1 {0x80} else {0};
        let bit = if apply_with_carry { carry } else { if self.get_f_carry() {0x80} else {0} };
        let result = (arg >> 1) | bit;

        self.set_f_half_carry(false);
        self.set_f_negative(false);
        self.set_f_zero(result == 0);
        self.set_f_carry(carry > 0);

        return result;
    }

    pub fn apply_add_u8_with_flags(&mut self, a: u8, b: u8, apply_with_carry: bool) -> u8 {
        let carry: u8 = if apply_with_carry { if self.get_f_carry() {1} else {0} } else { 0 };
        let result = a.wrapping_add(b).wrapping_add(carry);

        self.set_f_zero(result == 0);
        self.set_f_half_carry(((a & 0xF) + (b & 0xF) + carry) > 0xF);
        self.set_f_negative(false);
        self.set_f_carry((a as u16) + (b as u16) + (carry as u16) > 0xFF);

        return result;
    }

    pub fn apply_sub_u8_with_flags(&mut self, a: u8, b: u8, apply_with_carry: bool) -> u8 {
        let carry: u8 = if apply_with_carry { if self.get_f_carry() {1} else {0} } else { 0 };
        let result = a.wrapping_sub(b).wrapping_sub(carry);

        self.set_f_zero(result == 0);
        self.set_f_half_carry((a & 0x0F) < (b & 0x0F) + carry);
        self.set_f_negative(true);
        self.set_f_carry((a as i16 - b as i16 - carry as i16) < 0);

        return result;
    }

    pub fn apply_add_u16_with_flags(&mut self, a: u16, b: u16) -> u16 {
        let result: u16 = a.wrapping_add(b);

        self.set_f_half_carry((a & 0x07FF) + (b & 0x07FF) > 0x07FF);
        self.set_f_negative(false);
        self.set_f_carry(a > 0xFFFF - b);

        return result;
    }

    pub fn apply_add_i16_with_flags(&mut self, a:  i16, b: i16) -> u16 {
        let result: u16 = a.wrapping_add(b) as u16;

        self.set_f_negative(false);
        self.set_f_zero(false);
        self.set_f_half_carry((a & 0x000F) + (b & 0x000F) > 0x000F);
        self.set_f_carry((a & 0x00FF) + (b & 0x00FF) > 0x00FF);

        return result;
    }

    pub fn apply_and_u8_with_flags(&mut self, a: u8, b: u8) -> u8 {
        let result = a & b;

        self.set_f_zero(result == 0);
        self.set_f_half_carry(true);
        self.set_f_carry(false);
        self.set_f_negative(false);

        return result;
    }

    pub fn apply_or_u8_with_flags(&mut self, a: u8, b: u8) -> u8 {
        let result = a | b;

        self.set_f_zero(result == 0);
        self.set_f_half_carry(false);
        self.set_f_carry(false);
        self.set_f_negative(false);

        return result;
    }

    pub fn apply_xor_u8_with_flags(&mut self, a: u8, b: u8) -> u8 {
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
        self.sp = self.sp.wrapping_sub(1);
        mmu.write_byte(self.sp, value);
    }

    pub fn push_word(&mut self, mmu: &mut Mmu, value: u16) {
        let low = (value & 0xFF) as u8;
        let high = (value >> 8) as u8;

        self.push_byte(mmu, high);
        self.push_byte(mmu, low);
    }

    pub fn pop_byte(&mut self, mmu: &mut Mmu) -> u8 {
        let value = mmu.read_byte(self.sp);
        self.sp = self.sp.wrapping_add(1);
        return value;
    }

    pub fn pop_word(&mut self, mmu: &mut Mmu) -> u16 {
        let low = self.pop_byte(mmu) as u16;
        let high = self.pop_byte(mmu) as u16;
        return high << 8 | low;
    }
}

