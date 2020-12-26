use crate::cpu::Cpu;
use crate::mmu::Mmu;

#[allow(unreachable_patterns)]
pub fn execute_operation(opcode: u8, cpu: &mut Cpu, mmu: &mut Mmu) -> () {
    match opcode {
        0x00 => {
            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x01 => {
            cpu.set_bc(mmu.read_word(cpu.pc + 1));
            cpu.pc += 3;
            cpu.cycles += 3;
        }
        0x02 => {
            mmu.write_byte(cpu.get_bc(), cpu.a);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x03 => {
            cpu.set_bc(cpu.get_bc().wrapping_add(1));
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x04 => {
            cpu.b = cpu.apply_inc_u8_with_flags(cpu.b);
            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x05 => {
            cpu.b = cpu.apply_dec_u8_with_flags(cpu.b);
            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x06 => {
            cpu.b = mmu.read_byte(cpu.pc + 1);
            cpu.pc += 2;
            cpu.cycles += 2;
        }
        0x07 => {
            cpu.a = cpu.apply_rotate_left_with_flags(cpu.a, true);
            cpu.set_f_zero(false);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x08 => {
            let address = mmu.read_word(cpu.pc + 1);
            mmu.write_word(address, cpu.sp);

            cpu.pc += 3;
            cpu.cycles += 5;
        }
        0x09 => {
            let value = cpu.apply_add_u16_with_flags(cpu.get_hl(), cpu.get_bc());
            cpu.set_hl(value);

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x0A => {
            cpu.a = mmu.read_byte(cpu.get_bc());

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x0B => {
            let value: u16 = cpu.get_bc().wrapping_sub(1);
            cpu.set_bc(value);

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x0C => {
            cpu.c = cpu.apply_inc_u8_with_flags(cpu.c);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x0D => {
            cpu.c = cpu.apply_dec_u8_with_flags(cpu.c);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x0E => {
            cpu.c = mmu.read_byte(cpu.pc + 1);

            cpu.pc += 2;
            cpu.cycles += 2;
        }
        0x0F => {
            cpu.a = cpu.apply_rotate_right_with_flags(cpu.a, true);
            cpu.set_f_zero(false);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x10 => {
            mmu.toggle_speed();

            cpu.pc += 2;
            cpu.cycles += 1;
        }
        0x11 => {
            cpu.set_de(mmu.read_word(cpu.pc + 1));

            cpu.pc += 3;
            cpu.cycles += 3;
        }
        0x12 => {
            mmu.write_byte(cpu.get_de(), cpu.a);

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x13 => {
            cpu.set_de(cpu.get_de().wrapping_add(1));

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x14 => {
            cpu.d = cpu.apply_inc_u8_with_flags(cpu.d);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x15 => {
            cpu.d = cpu.apply_dec_u8_with_flags(cpu.d);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x16 => {
            cpu.d = mmu.read_byte(cpu.pc + 1);

            cpu.pc += 2;
            cpu.cycles += 2;
        }
        0x17 => {
            cpu.a = cpu.apply_rotate_left_with_flags(cpu.a, false);
            cpu.set_f_zero(false);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x18 => {
            let offset = mmu.read_byte(cpu.pc + 1) as i8;
            cpu.pc = ((cpu.pc as u32 as i32) + (offset as i32)) as u16;

            cpu.pc += 2;
            cpu.cycles += 3;
        }
        0x19 => {
            let value = cpu.apply_add_u16_with_flags(cpu.get_hl(), cpu.get_de());
            cpu.set_hl(value);

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x1A => {
            cpu.a = mmu.read_byte(cpu.get_de());

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x1B => {
            cpu.set_de(cpu.get_de().wrapping_sub(1));

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x1C => {
            cpu.e = cpu.apply_inc_u8_with_flags(cpu.e);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x1D => {
            cpu.e = cpu.apply_dec_u8_with_flags(cpu.e);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x1E => {
            cpu.e = mmu.read_byte(cpu.pc + 1);

            cpu.pc += 2;
            cpu.cycles += 2;
        }
        0x1F => {
            cpu.a = cpu.apply_rotate_right_with_flags(cpu.a, false);
            cpu.set_f_zero(false);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x20 => {
            if !cpu.get_f_zero() {
                let offset = mmu.read_byte(cpu.pc + 1) as i8;
                cpu.pc = ((cpu.pc as u32 as i32) + (offset as i32)) as u16;

                cpu.pc += 2;
                cpu.cycles += 3;
            } else {
                cpu.pc += 2;
                cpu.cycles += 2;
            }
        }
        0x21 => {
            let value = mmu.read_word(cpu.pc + 1);
            cpu.set_hl(value);

            cpu.pc += 3;
            cpu.cycles += 2;
        }
        0x22 => {
            mmu.write_byte(cpu.get_hl(), cpu.a);
            cpu.set_hl(cpu.get_hl().wrapping_add(1));

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x23 => {
            let value = cpu.get_hl().wrapping_add(1);
            cpu.set_hl(value);

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x24 => {
            cpu.h = cpu.apply_inc_u8_with_flags(cpu.h);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x25 => {
            cpu.h = cpu.apply_dec_u8_with_flags(cpu.h);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x26 => {
            cpu.h = mmu.read_byte(cpu.pc + 1);

            cpu.pc += 2;
            cpu.cycles += 2;
        }
        0x27 => {
            let mut a = cpu.a;
            let mut adjust = if cpu.get_f_carry() { 0x60 } else { 0x00 };
            if cpu.get_f_half_carry() { adjust |= 0x06; };
            if !cpu.get_f_substract() {
                if a & 0x0F > 0x09 { adjust |= 0x06; };
                if a > 0x99 { adjust |= 0x60; };
                a = a.wrapping_add(adjust);
            } else {
                a = a.wrapping_sub(adjust);
            }

            cpu.a = a;

            cpu.set_f_carry(adjust >= 0x60);
            cpu.set_f_half_carry(false);
            cpu.set_f_zero(a == 0);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x28 => {
            if cpu.get_f_zero() {
                let offset = mmu.read_byte(cpu.pc + 1) as i8;
                cpu.pc = ((cpu.pc as u32 as i32) + (offset as i32)) as u16;

                cpu.pc += 2;
                cpu.cycles += 3;
            } else {
                cpu.pc += 2;
                cpu.cycles += 2;
            }
        }
        0x29 => {
            let result = cpu.apply_add_u16_with_flags(cpu.get_hl(), cpu.get_hl());
            cpu.set_hl(result);

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x2A => {
            cpu.a = mmu.read_byte(cpu.get_hl());
            cpu.set_hl(cpu.get_hl().wrapping_add(1));

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x2B => {
            cpu.set_hl(cpu.get_hl().wrapping_sub(1));

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x2C => {
            cpu.l = cpu.apply_inc_u8_with_flags(cpu.l);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x2D => {
            cpu.l = cpu.apply_dec_u8_with_flags(cpu.l);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x2E => {
            cpu.l = mmu.read_byte(cpu.pc + 1);

            cpu.pc += 2;
            cpu.cycles += 2;
        }
        0x2F => {
            cpu.a = !cpu.a;
            cpu.set_f_half_carry(true);
            cpu.set_f_negative(true);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x30 => {
            if !cpu.get_f_carry() {
                let offset = mmu.read_byte(cpu.pc + 1) as i8;
                cpu.pc = ((cpu.pc as u32 as i32) + (offset as i32)) as u16;

                cpu.pc += 2;
                cpu.cycles += 3;
            } else {
                cpu.pc += 2;
                cpu.cycles += 2;
            }
        }
        0x31 => {
            cpu.sp = mmu.read_word(cpu.pc + 1);

            cpu.pc += 3;
            cpu.cycles += 3;
        }
        0x32 => {
            mmu.write_byte(cpu.get_hl(), cpu.a);
            cpu.set_hl(cpu.get_hl() - 1);

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x33 => {
            cpu.sp = cpu.sp.wrapping_add(1);

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x34 => {
            let addr = cpu.get_hl();
            let mut value = mmu.read_byte(addr);
            value = cpu.apply_inc_u8_with_flags(value);
            mmu.write_byte(addr, value);

            cpu.pc += 1;
            cpu.cycles += 3;
        }
        0x35 => {
            let addr = cpu.get_hl();
            let mut value = mmu.read_byte(addr);
            value = cpu.apply_dec_u8_with_flags(value);
            mmu.write_byte(addr, value);

            cpu.pc += 1;
            cpu.cycles += 3;
        }
        0x36 => {
            let value = mmu.read_byte(cpu.pc + 1);
            mmu.write_byte(cpu.get_hl(), value);

            cpu.pc += 2;
            cpu.cycles += 3;
        }
        0x37 => {
            cpu.set_f_carry(true);
            cpu.set_f_half_carry(false);
            cpu.set_f_negative(false);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x38 => {
            if cpu.get_f_carry() {
                let offset = mmu.read_byte(cpu.pc + 1) as i8;
                cpu.pc = ((cpu.pc as u32 as i32) + (offset as i32)) as u16;

                cpu.pc += 2;
                cpu.cycles += 3;
            } else {
                cpu.pc += 2;
                cpu.cycles += 2;
            }
        }
        0x39 => {
            let result = cpu.apply_add_u16_with_flags(cpu.get_hl(), cpu.sp);
            cpu.set_hl(result);

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x3A => {
            cpu.a = mmu.read_byte(cpu.get_hl());
            cpu.set_hl(cpu.get_hl() - 1);

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x3B => {
            cpu.sp = cpu.sp.wrapping_sub(1);

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x3C => {
            cpu.a = cpu.apply_inc_u8_with_flags(cpu.a);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x3D => {
            cpu.a = cpu.apply_dec_u8_with_flags(cpu.a);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x3E => {
            cpu.a = mmu.read_byte(cpu.pc + 1);

            cpu.pc += 2;
            cpu.cycles += 2;
        }
        0x3F => {
            cpu.set_f_carry(!cpu.get_f_carry());
            cpu.set_f_half_carry(false);
            cpu.set_f_negative(false);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x40 => {
            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x41 => {
            cpu.b = cpu.c;

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x42 => {
            cpu.b = cpu.d;

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x43 => {
            cpu.b = cpu.e;

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x44 => {
            cpu.b = cpu.h;

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x45 => {
            cpu.b = cpu.l;

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x46 => {
            cpu.b = mmu.read_byte(cpu.get_hl());

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x47 => {
            cpu.b = cpu.a;

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x48 => {
            cpu.c = cpu.b;

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x49 => {
            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x4A => {
            cpu.c = cpu.d;

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x4B => {
            cpu.c = cpu.e;

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x4C => {
            cpu.c = cpu.h;

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x4D => {
            cpu.c = cpu.l;

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x4E => {
            cpu.c = mmu.read_byte(cpu.get_hl());

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x4F => {
            cpu.c = cpu.a;

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x50 => {
            cpu.d = cpu.b;

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x51 => {
            cpu.d = cpu.c;

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x52 => {
            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x53 => {
            cpu.d = cpu.e;

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x54 => {
            cpu.d = cpu.h;

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x55 => {
            cpu.d = cpu.l;

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x56 => {
            cpu.d = mmu.read_byte(cpu.get_hl());

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x57 => {
            cpu.d = cpu.a;

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x58 => {
            cpu.e = cpu.b;

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x59 => {
            cpu.e = cpu.c;

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x5A => {
            cpu.e = cpu.d;

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x5B => {
            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x5C => {
            cpu.e = cpu.h;

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x5D => {
            cpu.e = cpu.l;

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x5E => {
            cpu.e = mmu.read_byte(cpu.get_hl());

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x5F => {
            cpu.e = cpu.a;

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x60 => {
            cpu.h = cpu.b;

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x61 => {
            cpu.h = cpu.c;

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x62 => {
            cpu.h = cpu.d;

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x63 => {
            cpu.h = cpu.e;

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x64 => {
            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x65 => {
            cpu.h = cpu.l;

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x66 => {
            cpu.h = mmu.read_byte(cpu.get_hl());

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x67 => {
            cpu.h = cpu.a;

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x68 => {
            cpu.l = cpu.b;

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x69 => {
            cpu.l = cpu.c;

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x6A => {
            cpu.l = cpu.d;

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x6B => {
            cpu.l = cpu.e;

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x6C => {
            cpu.l = cpu.h;

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x6D => {
            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x6E => {
            cpu.l = mmu.read_byte(cpu.get_hl());

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x6F => {
            cpu.l = cpu.a;

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x70 => {
            mmu.write_byte(cpu.get_hl(), cpu.b);

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x71 => {
            mmu.write_byte(cpu.get_hl(), cpu.c);

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x72 => {
            mmu.write_byte(cpu.get_hl(), cpu.d);

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x73 => {
            mmu.write_byte(cpu.get_hl(), cpu.e);

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x74 => {
            mmu.write_byte(cpu.get_hl(), cpu.h);

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x75 => {
            mmu.write_byte(cpu.get_hl(), cpu.l);

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x76 => {
            cpu.halted = true;

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x77 => {
            mmu.write_byte(cpu.get_hl(), cpu.a);

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x78 => {
            cpu.a = cpu.b;

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x79 => {
            cpu.a = cpu.c;

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x7A => {
            cpu.a = cpu.d;

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x7B => {
            cpu.a = cpu.e;

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x7C => {
            cpu.a = cpu.h;

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x7D => {
            cpu.a = cpu.l;

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x7E => {
            cpu.a = mmu.read_byte(cpu.get_hl());

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x7F => {
            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x80 => {
            cpu.a = cpu.apply_add_u8_with_flags(cpu.a, cpu.b, false);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x81 => {
            cpu.a = cpu.apply_add_u8_with_flags(cpu.a, cpu.c, false);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x82 => {
            cpu.a = cpu.apply_add_u8_with_flags(cpu.a, cpu.d, false);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x83 => {
            cpu.a = cpu.apply_add_u8_with_flags(cpu.a, cpu.e, false);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x84 => {
            cpu.a = cpu.apply_add_u8_with_flags(cpu.a, cpu.h, false);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x85 => {
            cpu.a = cpu.apply_add_u8_with_flags(cpu.a, cpu.l, false);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x86 => {
            let value = mmu.read_byte(cpu.get_hl());
            cpu.a = cpu.apply_add_u8_with_flags(cpu.a, value, false);

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x87 => {
            cpu.a = cpu.apply_add_u8_with_flags(cpu.a, cpu.a, false);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x88 => {
            cpu.a = cpu.apply_add_u8_with_flags(cpu.a, cpu.b, true);

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x89 => {
            cpu.a = cpu.apply_add_u8_with_flags(cpu.a, cpu.c, true);

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x8A => {
            cpu.a = cpu.apply_add_u8_with_flags(cpu.a, cpu.d, true);

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x8B => {
            cpu.a = cpu.apply_add_u8_with_flags(cpu.a, cpu.e, true);

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x8C => {
            cpu.a = cpu.apply_add_u8_with_flags(cpu.a, cpu.h, true);

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x8D => {
            cpu.a = cpu.apply_add_u8_with_flags(cpu.a, cpu.l, true);

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x8E => {
            let value = mmu.read_byte(cpu.get_hl());
            cpu.a = cpu.apply_add_u8_with_flags(cpu.a, value, true);

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x8F => {
            cpu.a = cpu.apply_add_u8_with_flags(cpu.a, cpu.a, true);

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x90 => {
            cpu.a = cpu.apply_sub_u8_with_flags(cpu.a, cpu.b, false);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x91 => {
            cpu.a = cpu.apply_sub_u8_with_flags(cpu.a, cpu.c, false);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x92 => {
            cpu.a = cpu.apply_sub_u8_with_flags(cpu.a, cpu.d, false);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x93 => {
            cpu.a = cpu.apply_sub_u8_with_flags(cpu.a, cpu.e, false);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x94 => {
            cpu.a = cpu.apply_sub_u8_with_flags(cpu.a, cpu.h, false);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x95 => {
            cpu.a = cpu.apply_sub_u8_with_flags(cpu.a, cpu.l, false);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x96 => {
            let value = mmu.read_byte(cpu.get_hl());
            cpu.a = cpu.apply_sub_u8_with_flags(cpu.a, value, false);

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x97 => {
            cpu.a = cpu.apply_sub_u8_with_flags(cpu.a, cpu.a, false);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x98 => {
            cpu.a = cpu.apply_sub_u8_with_flags(cpu.a, cpu.b, true);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x99 => {
            cpu.a = cpu.apply_sub_u8_with_flags(cpu.a, cpu.c, true);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x9A => {
            cpu.a = cpu.apply_sub_u8_with_flags(cpu.a, cpu.d, true);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x9B => {
            cpu.a = cpu.apply_sub_u8_with_flags(cpu.a, cpu.e, true);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x9C => {
            cpu.a = cpu.apply_sub_u8_with_flags(cpu.a, cpu.h, true);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x9D => {
            cpu.a = cpu.apply_sub_u8_with_flags(cpu.a, cpu.l, true);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0x9E => {
            let value = mmu.read_byte(cpu.get_hl());
            cpu.a = cpu.apply_sub_u8_with_flags(cpu.a, value, true);

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x9F => {
            cpu.a = cpu.apply_sub_u8_with_flags(cpu.a, cpu.a, true);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0xA0 => {
            cpu.a = cpu.apply_and_u8_with_flags(cpu.a, cpu.b);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0xA1 => {
            cpu.a = cpu.apply_and_u8_with_flags(cpu.a, cpu.c);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0xA2 => {
            cpu.a = cpu.apply_and_u8_with_flags(cpu.a, cpu.d);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0xA3 => {
            cpu.a = cpu.apply_and_u8_with_flags(cpu.a, cpu.e);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0xA4 => {
            cpu.a = cpu.apply_and_u8_with_flags(cpu.a, cpu.h);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0xA5 => {
            cpu.a = cpu.apply_and_u8_with_flags(cpu.a, cpu.l);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0xA6 => {
            let value = mmu.read_byte(cpu.get_hl());
            cpu.a = cpu.apply_and_u8_with_flags(cpu.a, value);

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xA7 => {
            cpu.a = cpu.apply_and_u8_with_flags(cpu.a, cpu.a);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0xA8 => {
            cpu.a = cpu.apply_xor_u8_with_flags(cpu.a, cpu.b);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0xA9 => {
            cpu.a = cpu.apply_xor_u8_with_flags(cpu.a, cpu.c);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0xAA => {
            cpu.a = cpu.apply_xor_u8_with_flags(cpu.a, cpu.d);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0xAB => {
            cpu.a = cpu.apply_xor_u8_with_flags(cpu.a, cpu.e);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0xAC => {
            cpu.a = cpu.apply_xor_u8_with_flags(cpu.a, cpu.h);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0xAD => {
            cpu.a = cpu.apply_xor_u8_with_flags(cpu.a, cpu.l);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0xAE => {
            let value = mmu.read_byte(cpu.get_hl());
            cpu.a = cpu.apply_xor_u8_with_flags(cpu.a, value);

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xAF => {
            cpu.a = cpu.apply_xor_u8_with_flags(cpu.a, cpu.a);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0xB0 => {
            cpu.a = cpu.apply_or_u8_with_flags(cpu.a, cpu.b);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0xB1 => {
            cpu.a = cpu.apply_or_u8_with_flags(cpu.a, cpu.c);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0xB2 => {
            cpu.a = cpu.apply_or_u8_with_flags(cpu.a, cpu.d);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0xB3 => {
            cpu.a = cpu.apply_or_u8_with_flags(cpu.a, cpu.e);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0xB4 => {
            cpu.a = cpu.apply_or_u8_with_flags(cpu.a, cpu.h);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0xB5 => {
            cpu.a = cpu.apply_or_u8_with_flags(cpu.a, cpu.l);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0xB6 => {
            let value = mmu.read_byte(cpu.get_hl());
            cpu.a = cpu.apply_or_u8_with_flags(cpu.a, value);

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xB7 => {
            cpu.a = cpu.apply_or_u8_with_flags(cpu.a, cpu.a);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0xB8 => {
            cpu.apply_sub_u8_with_flags(cpu.a, cpu.b, false);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0xB9 => {
            cpu.apply_sub_u8_with_flags(cpu.a, cpu.c, false);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0xBA => {
            cpu.apply_sub_u8_with_flags(cpu.a, cpu.d, false);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0xBB => {
            cpu.apply_sub_u8_with_flags(cpu.a, cpu.e, false);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0xBC => {
            cpu.apply_sub_u8_with_flags(cpu.a, cpu.h, false);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0xBD => {
            cpu.apply_sub_u8_with_flags(cpu.a, cpu.l, false);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0xBE => {
            let value = mmu.read_byte(cpu.get_hl());
            cpu.apply_sub_u8_with_flags(cpu.a, value, false);

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xBF => {
            cpu.apply_sub_u8_with_flags(cpu.a, cpu.a, false);

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0xC0 => {
            if !cpu.get_f_zero() {
                cpu.pc = cpu.pop_word(mmu);
                cpu.cycles += 5;
            } else {
                cpu.pc += 1;
                cpu.cycles += 2;
            }
        }
        0xC1 => {
            let value = cpu.pop_word(mmu);
            cpu.set_bc(value);

            cpu.pc += 1;
            cpu.cycles += 3;
        }
        0xC2 => {
            if !cpu.get_f_zero() {
                cpu.pc = mmu.read_word(cpu.pc + 1);

                cpu.cycles += 4;
            } else {
                cpu.pc += 3;
                cpu.cycles += 3;
            }
        }
        0xC3 => {
            cpu.pc = mmu.read_word(cpu.pc + 1);

            cpu.cycles += 4;
        }
        0xC4 => {
            if !cpu.get_f_zero() {
                cpu.push_word(mmu, cpu.pc + 3);
                cpu.pc = mmu.read_word(cpu.pc + 1);

                cpu.cycles += 6;
            } else {
                cpu.pc += 3;
                cpu.cycles += 3;
            }
        }
        0xC5 => {
            cpu.push_word(mmu, cpu.get_bc());

            cpu.pc += 1;
            cpu.cycles += 4;
        }
        0xC6 => {
            let value = mmu.read_byte(cpu.pc + 1);
            cpu.a = cpu.apply_add_u8_with_flags(cpu.a, value, false);


            cpu.pc += 2;
            cpu.cycles += 2;
        }
        0xC7 => {
            cpu.push_word(mmu, cpu.pc + 1);
            cpu.pc = 0x00;

            cpu.cycles += 4;
        }
        0xC8 => {
            if cpu.get_f_zero() {
                cpu.pc = cpu.pop_word(mmu);

                cpu.cycles += 5;
            } else {
                cpu.pc += 1;
                cpu.cycles += 2;
            }
        }
        0xC9 => {
            cpu.pc = cpu.pop_word(mmu);
            cpu.cycles += 4;
        }
        0xCA => {
            if cpu.get_f_zero() {
                cpu.pc = mmu.read_word(cpu.pc + 1);

                cpu.cycles += 4;
            } else {
                cpu.pc += 3;
                cpu.cycles += 3;
            }
        }
        0xCB => {
            op_cb(cpu, mmu);
        }
        0xCC => {
            if cpu.get_f_zero() {
                cpu.push_word(mmu,cpu.pc + 3);
                cpu.pc = mmu.read_word(cpu.pc + 1);

                cpu.cycles += 6;
            } else {
                cpu.pc += 3;
                cpu.cycles += 3;
            }
        }
        0xCD => {
            cpu.push_word(mmu, cpu.pc + 3);
            cpu.pc = mmu.read_word(cpu.pc + 1);

            cpu.cycles += 6;
        }
        0xCE => {
            let value = mmu.read_byte(cpu.pc + 1);
            cpu.a = cpu.apply_add_u8_with_flags(cpu.a, value, true);

            cpu.pc += 2;
            cpu.cycles += 2;
        }
        0xCF => {
            cpu.push_word(mmu, cpu.pc + 1);
            cpu.pc = 0x08;

            cpu.cycles += 4;
        }
        0xD0 => {
            if !cpu.get_f_carry() {
                cpu.pc = cpu.pop_word(mmu);

                cpu.cycles += 5;
            } else {
                cpu.pc += 1;
                cpu.cycles += 2;
            }
        }
        0xD1 => {
            let value = cpu.pop_word(mmu);
            cpu.set_de(value);

            cpu.pc += 1;
            cpu.cycles += 3;
        }
        0xD2 => {
            if !cpu.get_f_carry() {
                cpu.pc = mmu.read_word(cpu.pc + 1);
                cpu.cycles += 4;
            } else {
                cpu.pc += 3;
                cpu.cycles += 3;
            }
        }
        0xD3 => {
            panic!("not implemented");
        }
        0xD4 => {
            if !cpu.get_f_carry() {
                cpu.push_word(mmu, cpu.pc + 3);
                cpu.pc = mmu.read_word(cpu.pc + 1);
                cpu.cycles += 6;
            } else {
                cpu.pc += 3;
                cpu.cycles += 3;
            }
        }
        0xD5 => {
            cpu.push_word(mmu, cpu.get_de());

            cpu.pc += 1;
            cpu.cycles += 4;
        }
        0xD6 => {
            let value = mmu.read_byte(cpu.pc + 1);
            cpu.a = cpu.apply_sub_u8_with_flags(cpu.a, value, false);

            cpu.pc += 2;
            cpu.cycles += 2;
        }
        0xD7 => {
            cpu.push_word(mmu, cpu.pc + 1);
            cpu.pc = 0x10;

            cpu.cycles += 4;
        }
        0xD8 => {
            if cpu.get_f_carry() {
                cpu.pc = cpu.pop_word(mmu);

                cpu.cycles += 5;
            } else {
                cpu.pc += 1;
                cpu.cycles += 2;
            }
        }
        0xD9 => {
            cpu.pc = cpu.pop_word(mmu);
            cpu.enable_interrupt_counter = 1;

            cpu.cycles += 4;
        }
        0xDA => {
            if cpu.get_f_carry() {
                cpu.pc = mmu.read_word(cpu.pc + 1);

                cpu.cycles += 4;
            } else {

                cpu.pc += 3;
                cpu.cycles += 3;
            }
        }
        0xDB => {
            panic!("not implemented");
        }
        0xDC => {
            if cpu.get_f_carry() {
                cpu.push_word(mmu, cpu.pc + 3);
                cpu.pc = mmu.read_word(cpu.pc + 1);
                cpu.cycles += 6;
            } else {
                cpu.pc += 3;
                cpu.cycles += 3;
            }
        }
        0xDD => {
            panic!("not implemented");
        }
        0xDE => {
            let value = mmu.read_byte(cpu.pc + 1);
            cpu.a = cpu.apply_sub_u8_with_flags(cpu.a, value, true);

            cpu.pc += 2;
            cpu.cycles += 2;
        }
        0xDF => {
            cpu.push_word(mmu, cpu.pc + 1);
            cpu.pc = 0x18;

            cpu.cycles += 4;
        }
        0xE0 => {
            let value = mmu.read_byte(cpu.pc + 1) as u16;
            let a = 0xFF00 | value;

            mmu.write_byte(a, cpu.a);

            cpu.pc += 2;
            cpu.cycles += 3;
        }
        0xE1 => {
            let value = cpu.pop_word(mmu);
            cpu.set_hl(value);

            cpu.pc += 1;
            cpu.cycles += 3;
        }
        0xE2 => {
            let address = 0xFF00 | (cpu.c as u16);
            mmu.write_byte(address, cpu.a);

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xE3 => {
            panic!("not implemented");
        }
        0xE4 => {
            panic!("not implemented");
        }
        0xE5 => {
            cpu.push_word(mmu, cpu.get_hl());

            cpu.pc += 1;
            cpu.cycles += 4;
        }
        0xE6 => {
            let value = mmu.read_byte(cpu.pc + 1);
            cpu.a = cpu.apply_and_u8_with_flags(cpu.a, value);

            cpu.pc += 2;
            cpu.cycles += 2;
        }
        0xE7 => {
            cpu.push_word(mmu, cpu.pc + 1);
            cpu.pc = 0x20;

            cpu.cycles += 4;
        }
        0xE8 => {
            let value = mmu.read_byte(cpu.pc + 1) as i8 as i16;
            let result = cpu.apply_add_i16_with_flags(cpu.sp as i16, value);
            cpu.sp = result;

            cpu.pc += 2;
            cpu.cycles += 4;
        }
        0xE9 => {
            cpu.pc = cpu.get_hl();

            cpu.cycles += 1;
        }
        0xEA => {
            let a = mmu.read_word(cpu.pc + 1);
            mmu.write_byte(a, cpu.a);

            cpu.pc += 3;
            cpu.cycles += 4;
        }
        0xEB => {
            panic!("not implemented");
        }
        0xEC => {
            panic!("not implemented");
        }
        0xED => {
            panic!("not implemented");
        }
        0xEE => {
            let value = mmu.read_byte(cpu.pc + 1);
            cpu.a = cpu.apply_xor_u8_with_flags(cpu.a, value);

            cpu.pc += 2;
            cpu.cycles += 2;
        }
        0xEF => {
            cpu.push_word(mmu, cpu.pc + 1);
            cpu.pc = 0x28;

            cpu.cycles += 4;
        }
        0xF0 => {
            let address = 0xFF00 | mmu.read_byte(cpu.pc + 1) as u16;
            cpu.a = mmu.read_byte(address);

            cpu.pc += 2;
            cpu.cycles += 3;
        }
        0xF1 => {
            let value = cpu.pop_word(mmu) & 0xFFF0;
            cpu.set_af(value);

            cpu.pc += 1;
            cpu.cycles += 3;
        }
        0xF2 => {
            let address = 0xFF00 | cpu.c as u16;
            cpu.a = mmu.read_byte(address);

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xF3 => {
            // Schedules interrupt handling to be enabled after the next machine cycle
            cpu.disable_interrupt_counter = 2;

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0xF4 => {
            panic!("not implemented");
        }
        0xF5 => {
            cpu.push_word(mmu, cpu.get_af());

            cpu.pc += 1;
            cpu.cycles += 4;
        }
        0xF6 => {
            let value = mmu.read_byte(cpu.pc + 1);
            cpu.a = cpu.apply_or_u8_with_flags(cpu.a, value);

            cpu.pc += 2;
            cpu.cycles += 2;
        }
        0xF7 => {
            cpu.push_word(mmu,cpu.pc + 1);
            cpu.pc = 0x30;

            cpu.cycles += 4;
        }
        0xF8 => {
            let value = mmu.read_byte(cpu.pc + 1) as i8 as i16;
            let result = cpu.apply_add_i16_with_flags(cpu.sp as i16, value);
            cpu.set_hl(result);

            cpu.pc += 2;
            cpu.cycles += 3;
        }
        0xF9 => {
            cpu.sp = cpu.get_hl();

            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xFA => {
            let address = mmu.read_word(cpu.pc + 1);
            cpu.a = mmu.read_byte(address);

            cpu.pc += 3;
            cpu.cycles += 4;
        }
        0xFB => {
            cpu.enable_interrupt_counter = 2;

            cpu.pc += 1;
            cpu.cycles += 1;
        }
        0xFC => {
            panic!("not implemented");
        }
        0xFD => {
            panic!("not implemented");
        }
        0xFE => {
            let value = mmu.read_byte(cpu.pc + 1);
            cpu.apply_sub_u8_with_flags(cpu.a, value, false);

            cpu.pc += 2;
            cpu.cycles += 2;
        }
        0xFF => {
            cpu.push_word(mmu,cpu.pc + 1);
            cpu.pc = 0x38;

            cpu.cycles += 4;
        }
        _ => { panic!("opcode not found {}", opcode) }
    }
}

#[allow(unreachable_patterns)]
pub fn op_cb(cpu: &mut Cpu, mmu: &mut Mmu) {
    let opcode = mmu.read_byte(cpu.pc + 1);

    cpu.opcode = (cpu.opcode << 8) as u16 | opcode as u16;
    cpu.pc += 1;
    match opcode {
        0x00 => {
            cpu.b = cpu.apply_rotate_left_with_flags(cpu.b, true);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x01 => {
            cpu.c = cpu.apply_rotate_left_with_flags(cpu.c, true);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x02 => {
            cpu.d = cpu.apply_rotate_left_with_flags(cpu.d, true);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x03 => {
            cpu.e = cpu.apply_rotate_left_with_flags(cpu.e, true);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x04 => {
            cpu.h = cpu.apply_rotate_left_with_flags(cpu.h, true);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x05 => {
            cpu.l = cpu.apply_rotate_left_with_flags(cpu.l, true);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x06 => {
            let address = cpu.get_hl();
            let value = mmu.read_byte(address);

            let result = cpu.apply_rotate_left_with_flags(value, true);
            mmu.write_byte(address, result);

            cpu.pc += 1;
            cpu.cycles += 4;
        }
        0x07 => {
            cpu.a = cpu.apply_rotate_left_with_flags(cpu.a, true);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x08 => {
            cpu.b = cpu.apply_rotate_right_with_flags(cpu.b, true);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x09 => {
            cpu.c = cpu.apply_rotate_right_with_flags(cpu.c, true);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x0A => {
            cpu.d = cpu.apply_rotate_right_with_flags(cpu.d, true);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x0B => {
            cpu.e = cpu.apply_rotate_right_with_flags(cpu.e, true);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x0C => {
            cpu.h = cpu.apply_rotate_right_with_flags(cpu.h, true);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x0D => {
            cpu.l = cpu.apply_rotate_right_with_flags(cpu.l, true);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x0E => {
            let address = cpu.get_hl();
            let value = mmu.read_byte(address);

            let result = cpu.apply_rotate_right_with_flags(value, true);
            mmu.write_byte(address, result);

            cpu.pc += 1;
            cpu.cycles += 4;
        }
        0x0F => {
            cpu.a = cpu.apply_rotate_right_with_flags(cpu.a, true);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x10 => {
            cpu.b = cpu.apply_rotate_left_with_flags(cpu.b, false);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x11 => {
            cpu.c = cpu.apply_rotate_left_with_flags(cpu.c, false);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x12 => {
            cpu.d = cpu.apply_rotate_left_with_flags(cpu.d, false);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x13 => {
            cpu.e = cpu.apply_rotate_left_with_flags(cpu.e, false);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x14 => {
            cpu.h = cpu.apply_rotate_left_with_flags(cpu.h, false);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x15 => {
            cpu.l = cpu.apply_rotate_left_with_flags(cpu.l, false);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x16 => {
            let address = cpu.get_hl();
            let value = mmu.read_byte(address);

            let result = cpu.apply_rotate_left_with_flags(value, false);
            mmu.write_byte(address, result);

            cpu.pc += 1;
            cpu.cycles += 4;
        }
        0x17 => {
            cpu.a = cpu.apply_rotate_left_with_flags(cpu.a, false);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x18 => {
            cpu.b = cpu.apply_rotate_right_with_flags(cpu.b, false);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x19 => {
            cpu.c = cpu.apply_rotate_right_with_flags(cpu.c, false);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x1A => {
            cpu.d = cpu.apply_rotate_right_with_flags(cpu.d, false);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x1B => {
            cpu.e = cpu.apply_rotate_right_with_flags(cpu.e, false);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x1C => {
            cpu.h = cpu.apply_rotate_right_with_flags(cpu.h, false);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x1D => {
            cpu.l = cpu.apply_rotate_right_with_flags(cpu.l, false);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x1E => {
            let address = cpu.get_hl();
            let value = mmu.read_byte(address);

            let result = cpu.apply_rotate_right_with_flags(value, false);
            mmu.write_byte(address, result);

            cpu.pc += 1;
            cpu.cycles += 4;
        }
        0x1F => {
            cpu.a = cpu.apply_rotate_right_with_flags(cpu.a, false);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x20 => {
            cpu.b = cpu.apply_shift_left_with_flags(cpu.b);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x21 => {
            cpu.c = cpu.apply_shift_left_with_flags(cpu.c);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x22 => {
            cpu.d = cpu.apply_shift_left_with_flags(cpu.d);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x23 => {
            cpu.e = cpu.apply_shift_left_with_flags(cpu.e);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x24 => {
            cpu.h = cpu.apply_shift_left_with_flags(cpu.h);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x25 => {
            cpu.l = cpu.apply_shift_left_with_flags(cpu.l);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x26 => {
            let address = cpu.get_hl();
            let value = mmu.read_byte(address);

            let result = cpu.apply_shift_left_with_flags(value);
            mmu.write_byte(address, result);

            cpu.pc += 1;
            cpu.cycles += 4;
        }
        0x27 => {
            cpu.a = cpu.apply_shift_left_with_flags(cpu.a);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x28 => {
            cpu.b = cpu.apply_shift_right_with_flags(cpu.b, true);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x29 => {
            cpu.c = cpu.apply_shift_right_with_flags(cpu.c, true);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x2A => {
            cpu.d = cpu.apply_shift_right_with_flags(cpu.d, true);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x2B => {
            cpu.e = cpu.apply_shift_right_with_flags(cpu.e, true);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x2C => {
            cpu.h = cpu.apply_shift_right_with_flags(cpu.h, true);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x2D => {
            cpu.l = cpu.apply_shift_right_with_flags(cpu.l, true);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x2E => {
            let address = cpu.get_hl();
            let value = mmu.read_byte(address);

            let result = cpu.apply_shift_right_with_flags(value, true);
            mmu.write_byte(address, result);

            cpu.pc += 1;
            cpu.cycles += 4;
        }
        0x2F => {
            cpu.a = cpu.apply_shift_right_with_flags(cpu.a, true);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x30 => {
            cpu.b = cpu.apply_swap_bytes(cpu.b);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x31 => {
            cpu.c = cpu.apply_swap_bytes(cpu.c);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x32 => {
            cpu.d = cpu.apply_swap_bytes(cpu.d);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x33 => {
            cpu.e = cpu.apply_swap_bytes(cpu.e);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x34 => {
            cpu.h = cpu.apply_swap_bytes(cpu.h);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x35 => {
            cpu.l = cpu.apply_swap_bytes(cpu.l);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x36 => {
            let address = cpu.get_hl();
            let value = mmu.read_byte(address);

            let result = cpu.apply_swap_bytes(value);
            mmu.write_byte(address, result);

            cpu.pc += 1;
            cpu.cycles += 4;
        }
        0x37 => {
            cpu.a = cpu.apply_swap_bytes(cpu.a);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x38 => {
            cpu.b = cpu.apply_shift_right_with_flags(cpu.b, false);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x39 => {
            cpu.c = cpu.apply_shift_right_with_flags(cpu.c, false);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x3A => {
            cpu.d = cpu.apply_shift_right_with_flags(cpu.d, false);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x3B => {
            cpu.e = cpu.apply_shift_right_with_flags(cpu.e, false);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x3C => {
            cpu.h = cpu.apply_shift_right_with_flags(cpu.h, false);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x3D => {
            cpu.l = cpu.apply_shift_right_with_flags(cpu.l, false);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x3E => {
            let address = cpu.get_hl();
            let value = mmu.read_byte(address);

            let result = cpu.apply_shift_right_with_flags(value, false);
            mmu.write_byte(address, result);

            cpu.pc += 1;
            cpu.cycles += 4;
        }
        0x3F => {
            cpu.a = cpu.apply_shift_right_with_flags(cpu.a, false);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x40 => {
            cpu.apply_bit_test(cpu.b, 0);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x41 => {
            cpu.apply_bit_test(cpu.c, 0);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x42 => {
            cpu.apply_bit_test(cpu.d, 0);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x43 => {
            cpu.apply_bit_test(cpu.e, 0);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x44 => {
            cpu.apply_bit_test(cpu.h, 0);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x45 => {
            cpu.apply_bit_test(cpu.l, 0);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x46 => {
            let address = cpu.get_hl();
            let value = mmu.read_byte(address);

            cpu.apply_bit_test(value, 0);

            cpu.pc += 1;
            cpu.cycles += 4;
        }
        0x47 => {
            cpu.apply_bit_test(cpu.a, 0);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x48 => {
            cpu.apply_bit_test(cpu.b, 1);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x49 => {
            cpu.apply_bit_test(cpu.c, 1);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x4A => {
            cpu.apply_bit_test(cpu.d, 1);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x4B => {
            cpu.apply_bit_test(cpu.e, 1);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x4C => {
            cpu.apply_bit_test(cpu.h, 1);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x4D => {
            cpu.apply_bit_test(cpu.l, 1);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x4E => {
            let address = cpu.get_hl();
            let value = mmu.read_byte(address);

            cpu.apply_bit_test(value, 1);

            cpu.pc += 1;
            cpu.cycles += 4;
        }
        0x4F => {
            cpu.apply_bit_test(cpu.a, 1);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x50 => {
            cpu.apply_bit_test(cpu.b, 2);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x51 => {
            cpu.apply_bit_test(cpu.c, 2);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x52 => {
            cpu.apply_bit_test(cpu.d, 2);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x53 => {
            cpu.apply_bit_test(cpu.e, 2);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x54 => {
            cpu.apply_bit_test(cpu.h, 2);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x55 => {
            cpu.apply_bit_test(cpu.l, 2);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x56 => {
            let address = cpu.get_hl();
            let value = mmu.read_byte(address);

            cpu.apply_bit_test(value, 2);

            cpu.pc += 1;
            cpu.cycles += 4;
        }
        0x57 => {
            cpu.apply_bit_test(cpu.a, 2);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x58 => {
            cpu.apply_bit_test(cpu.b, 3);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x59 => {
            cpu.apply_bit_test(cpu.c, 3);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x5A => {
            cpu.apply_bit_test(cpu.d, 3);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x5B => {
            cpu.apply_bit_test(cpu.e, 3);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x5C => {
            cpu.apply_bit_test(cpu.h, 3);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x5D => {
            cpu.apply_bit_test(cpu.l, 3);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x5E => {
            let address = cpu.get_hl();
            let value = mmu.read_byte(address);

            cpu.apply_bit_test(value, 3);

            cpu.pc += 1;
            cpu.cycles += 4;
        }
        0x5F => {
            cpu.apply_bit_test(cpu.a, 3);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x60 => {
            cpu.apply_bit_test(cpu.b, 4);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x61 => {
            cpu.apply_bit_test(cpu.c, 4);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x62 => {
            cpu.apply_bit_test(cpu.d, 4);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x63 => {
            cpu.apply_bit_test(cpu.e, 4);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x64 => {
            cpu.apply_bit_test(cpu.h, 4);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x65 => {
            cpu.apply_bit_test(cpu.l, 4);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x66 => {
            let address = cpu.get_hl();
            let value = mmu.read_byte(address);

            cpu.apply_bit_test(value, 4);

            cpu.pc += 1;
            cpu.cycles += 4;
        }
        0x67 => {
            cpu.apply_bit_test(cpu.a, 4);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x68 => {
            cpu.apply_bit_test(cpu.b, 5);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x69 => {
            cpu.apply_bit_test(cpu.c, 5);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x6A => {
            cpu.apply_bit_test(cpu.d, 5);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x6B => {
            cpu.apply_bit_test(cpu.e, 5);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x6C => {
            cpu.apply_bit_test(cpu.h, 5);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x6D => {
            cpu.apply_bit_test(cpu.l, 5);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x6E => {
            let address = cpu.get_hl();
            let value = mmu.read_byte(address);

            cpu.apply_bit_test(value, 5);

            cpu.pc += 1;
            cpu.cycles += 4;
        }
        0x6F => {
            cpu.apply_bit_test(cpu.a, 5);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x70 => {
            cpu.apply_bit_test(cpu.b, 6);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x71 => {
            cpu.apply_bit_test(cpu.c, 6);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x72 => {
            cpu.apply_bit_test(cpu.d, 6);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x73 => {
            cpu.apply_bit_test(cpu.e, 6);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x74 => {
            cpu.apply_bit_test(cpu.h, 6);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x75 => {
            cpu.apply_bit_test(cpu.l, 6);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x76 => {
            let address = cpu.get_hl();
            let value = mmu.read_byte(address);

            cpu.apply_bit_test(value, 6);

            cpu.pc += 1;
            cpu.cycles += 4;
        }
        0x77 => {
            cpu.apply_bit_test(cpu.a, 6);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x78 => {
            cpu.apply_bit_test(cpu.b, 7);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x79 => {
            cpu.apply_bit_test(cpu.c, 7);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x7A => {
            cpu.apply_bit_test(cpu.d, 7);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x7B => {
            cpu.apply_bit_test(cpu.e, 7);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x7C => {
            cpu.apply_bit_test(cpu.h, 7);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x7D => {
            cpu.apply_bit_test(cpu.l, 7);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x7E => {
            let address = cpu.get_hl();
            let value = mmu.read_byte(address);

            cpu.apply_bit_test(value, 7);

            cpu.pc += 1;
            cpu.cycles += 4;
        }
        0x7F => {
            cpu.apply_bit_test(cpu.a, 7);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x80 => {
            cpu.b = cpu.b & !(1 << 0);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x81 => {
            cpu.c = cpu.c & !(1 << 0);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x82 => {
            cpu.d = cpu.d & !(1 << 0);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x83 => {
            cpu.e = cpu.e & !(1 << 0);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x84 => {
            cpu.h = cpu.h & !(1 << 0);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x85 => {
            cpu.l = cpu.l & !(1 << 0);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x86 => {
            let address = cpu.get_hl();
            let value = mmu.read_byte(address);

            mmu.write_byte(address, value & !(1 << 0));

            cpu.pc += 1;
            cpu.cycles += 4;
        }
        0x87 => {
            cpu.a = cpu.a & !(1 << 0);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x88 => {
            cpu.b = cpu.b & !(1 << 1);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x89 => {
            cpu.c = cpu.c & !(1 << 1);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x8A => {
            cpu.d = cpu.d & !(1 << 1);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x8B => {
            cpu.e = cpu.e & !(1 << 1);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x8C => {
            cpu.h = cpu.h & !(1 << 1);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x8D => {
            cpu.l = cpu.l & !(1 << 1);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x8E => {
            let address = cpu.get_hl();
            let value = mmu.read_byte(address);

            mmu.write_byte(address, value & !(1 << 1));

            cpu.pc += 1;
            cpu.cycles += 4;
        }
        0x8F => {
            cpu.a = cpu.a & !(1 << 1);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x90 => {
            cpu.b = cpu.b & !(1 << 2);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x91 => {
            cpu.c = cpu.c & !(1 << 2);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x92 => {
            cpu.d = cpu.d & !(1 << 2);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x93 => {
            cpu.e = cpu.e & !(1 << 2);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x94 => {
            cpu.h = cpu.h & !(1 << 2);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x95 => {
            cpu.l = cpu.l & !(1 << 2);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x96 => {
            let address = cpu.get_hl();
            let value = mmu.read_byte(address);

            mmu.write_byte(address, value & !(1 << 2));

            cpu.pc += 1;
            cpu.cycles += 4;
        }
        0x97 => {
            cpu.a = cpu.a & !(1 << 2);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x98 => {
            cpu.b = cpu.b & !(1 << 3);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x99 => {
            cpu.c = cpu.c & !(1 << 3);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x9A => {
            cpu.d = cpu.d & !(1 << 3);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x9B => {
            cpu.e = cpu.e & !(1 << 3);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x9C => {
            cpu.h = cpu.h & !(1 << 3);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x9D => {
            cpu.l = cpu.l & !(1 << 3);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x9E => {
            let address = cpu.get_hl();
            let value = mmu.read_byte(address);

            mmu.write_byte(address, value & !(1 << 3));

            cpu.pc += 1;
            cpu.cycles += 4;
        }
        0x9F => {
            cpu.a = cpu.a & !(1 << 3);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xA0 => {
            cpu.b = cpu.b & !(1 << 4);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xA1 => {
            cpu.c = cpu.c & !(1 << 4);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xA2 => {
            cpu.d = cpu.d & !(1 << 4);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xA3 => {
            cpu.e = cpu.e & !(1 << 4);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xA4 => {
            cpu.h = cpu.h & !(1 << 4);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xA5 => {
            cpu.l = cpu.l & !(1 << 4);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xA6 => {
            let address = cpu.get_hl();
            let value = mmu.read_byte(address);

            mmu.write_byte(address, value & !(1 << 4));

            cpu.pc += 1;
            cpu.cycles += 4;
        }
        0xA7 => {
            cpu.a = cpu.a & !(1 << 4);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xA8 => {
            cpu.b = cpu.b & !(1 << 5);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xA9 => {
            cpu.c = cpu.c & !(1 << 5);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xAA => {
            cpu.d = cpu.d & !(1 << 5);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xAB => {
            cpu.e = cpu.e & !(1 << 5);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xAC => {
            cpu.h = cpu.h & !(1 << 5);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xAD => {
            cpu.l = cpu.l & !(1 << 5);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xAE => {
            let address = cpu.get_hl();
            let value = mmu.read_byte(address);

            mmu.write_byte(address, value & !(1 << 5));

            cpu.pc += 1;
            cpu.cycles += 4;
        }
        0xAF => {
            cpu.a = cpu.a & !(1 << 5);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xB0 => {
            cpu.b = cpu.b & !(1 << 6);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xB1 => {
            cpu.c = cpu.c & !(1 << 6);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xB2 => {
            cpu.d = cpu.d & !(1 << 6);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xB3 => {
            cpu.e = cpu.e & !(1 << 6);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xB4 => {
            cpu.h = cpu.h & !(1 << 6);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xB5 => {
            cpu.l = cpu.l & !(1 << 6);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xB6 => {
            let address = cpu.get_hl();
            let value = mmu.read_byte(address);

            mmu.write_byte(address, value & !(1 << 6));

            cpu.pc += 1;
            cpu.cycles += 4;
        }
        0xB7 => {
            cpu.a = cpu.a & !(1 << 6);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xB8 => {
            cpu.b = cpu.b & !(1 << 7);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xB9 => {
            cpu.c = cpu.c & !(1 << 7);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xBA => {
            cpu.d = cpu.d & !(1 << 7);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xBB => {
            cpu.e = cpu.e & !(1 << 7);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xBC => {
            cpu.h = cpu.h & !(1 << 7);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xBD => {
            cpu.l = cpu.l & !(1 << 7);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xBE => {
            let address = cpu.get_hl();
            let value = mmu.read_byte(address);

            mmu.write_byte(address, value & !(1 << 7));

            cpu.pc += 1;
            cpu.cycles += 4;
        }
        0xBF => {
            cpu.a = cpu.a & !(1 << 7);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xC0 => {
            cpu.b = cpu.b | (1 << 0);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xC1 => {
            cpu.c = cpu.c | (1 << 0);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xC2 => {
            cpu.d = cpu.d | (1 << 0);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xC3 => {
            cpu.e = cpu.e | (1 << 0);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xC4 => {
            cpu.h = cpu.h | (1 << 0);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xC5 => {
            cpu.l = cpu.l | (1 << 0);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xC6 => {
            let address = cpu.get_hl();
            let value = mmu.read_byte(address);

            mmu.write_byte(address, value | (1 << 0));

            cpu.pc += 1;
            cpu.cycles += 4;
        }
        0xC7 => {
            cpu.a = cpu.a | (1 << 0);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xC8 => {
            cpu.b = cpu.b | (1 << 1);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xC9 => {
            cpu.c = cpu.c | (1 << 1);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xCA => {
            cpu.d = cpu.d | (1 << 1);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xCB => {
            cpu.e = cpu.e | (1 << 1);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xCC => {
            cpu.h = cpu.h | (1 << 1);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xCD => {
            cpu.l = cpu.l | (1 << 1);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xCE => {
            let address = cpu.get_hl();
            let value = mmu.read_byte(address);

            mmu.write_byte(address, value | (1 << 1));

            cpu.pc += 1;
            cpu.cycles += 4;
        }
        0xCF => {
            cpu.a = cpu.a | (1 << 1);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xD0 => {
            cpu.b = cpu.b | (1 << 2);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xD1 => {
            cpu.c = cpu.c | (1 << 2);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xD2 => {
            cpu.d = cpu.d | (1 << 2);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xD3 => {
            cpu.e = cpu.e | (1 << 2);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xD4 => {
            cpu.h = cpu.h | (1 << 2);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xD5 => {
            cpu.l = cpu.l | (1 << 2);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xD6 => {
            let address = cpu.get_hl();
            let value = mmu.read_byte(address);

            mmu.write_byte(address, value | (1 << 2));

            cpu.pc += 1;
            cpu.cycles += 4;
        }
        0xD7 => {
            cpu.a = cpu.a | (1 << 2);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xD8 => {
            cpu.b = cpu.b | (1 << 3);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xD9 => {
            cpu.c = cpu.c | (1 << 3);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xDA => {
            cpu.d = cpu.d | (1 << 3);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xDB => {
            cpu.e = cpu.e | (1 << 3);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xDC => {
            cpu.h = cpu.h | (1 << 3);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xDD => {
            cpu.l = cpu.l | (1 << 3);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xDE => {
            let address = cpu.get_hl();
            let value = mmu.read_byte(address);

            mmu.write_byte(address, value | (1 << 3));

            cpu.pc += 1;
            cpu.cycles += 4;
        }
        0xDF => {
            cpu.a = cpu.a | (1 << 3);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xE0 => {
            cpu.b = cpu.b | (1 << 4);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xE1 => {
            cpu.c = cpu.c | (1 << 4);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xE2 => {
            cpu.d = cpu.d | (1 << 4);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xE3 => {
            cpu.e = cpu.e | (1 << 4);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xE4 => {
            cpu.h = cpu.h | (1 << 4);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xE5 => {
            cpu.l = cpu.l | (1 << 4);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xE6 => {
            let address = cpu.get_hl();
            let value = mmu.read_byte(address);

            mmu.write_byte(address, value | (1 << 4));

            cpu.pc += 1;
            cpu.cycles += 4;
        }
        0xE7 => {
            cpu.a = cpu.a | (1 << 4);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xE8 => {
            cpu.b = cpu.b | (1 << 5);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xE9 => {
            cpu.c = cpu.c | (1 << 5);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xEA => {
            cpu.d = cpu.d | (1 << 5);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xEB => {
            cpu.e = cpu.e | (1 << 5);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xEC => {
            cpu.h = cpu.h | (1 << 5);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xED => {
            cpu.l = cpu.l | (1 << 5);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xEE => {
            let address = cpu.get_hl();
            let value = mmu.read_byte(address);

            mmu.write_byte(address, value | (1 << 5));

            cpu.pc += 1;
            cpu.cycles += 4;
        }
        0xEF => {
            cpu.a = cpu.a | (1 << 5);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xF0 => {
            cpu.b = cpu.b | (1 << 6);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xF1 => {
            cpu.c = cpu.c | (1 << 6);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xF2 => {
            cpu.d = cpu.d | (1 << 6);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xF3 => {
            cpu.e = cpu.e | (1 << 6);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xF4 => {
            cpu.h = cpu.h | (1 << 6);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xF5 => {
            cpu.l = cpu.l | (1 << 6);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xF6 => {
            let address = cpu.get_hl();
            let value = mmu.read_byte(address);

            mmu.write_byte(address, value | (1 << 6));

            cpu.pc += 1;
            cpu.cycles += 4;
        }
        0xF7 => {
            cpu.a = cpu.a | (1 << 6);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xF8 => {
            cpu.b = cpu.b | (1 << 7);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xF9 => {
            cpu.c = cpu.c | (1 << 7);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xFA => {
            cpu.d = cpu.d | (1 << 7);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xFB => {
            cpu.e = cpu.e | (1 << 7);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xFC => {
            cpu.h = cpu.h | (1 << 7);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xFD => {
            cpu.l = cpu.l | (1 << 7);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0xFE => {
            let address = cpu.get_hl();
            let value = mmu.read_byte(address);

            mmu.write_byte(address, value | (1 << 7));

            cpu.pc += 1;
            cpu.cycles += 4;
        }
        0xFF => {
            cpu.a = cpu.a | (1 << 7);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        _ => { panic!("opcode not found {}", opcode) }
    }
}
