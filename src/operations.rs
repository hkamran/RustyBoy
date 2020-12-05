use crate::cpu::Cpu;
use crate::mmu::Mmu;

pub type OperationType = fn(cpu: &mut Cpu, bus: &mut Mmu) -> ();

#[allow(unreachable_patterns)]
pub fn get(opcode: u8) -> OperationType {
    match opcode {
        0x00 => { op_00 }
        0x01 => { op_01 }
        0x02 => { op_02 }
        0x03 => { op_03 }
        0x04 => { op_04 }
        0x05 => { op_05 }
        0x06 => { op_06 }
        0x07 => { op_07 }
        0x08 => { op_08 }
        0x09 => { op_09 }
        0x0a => { op_0a }
        0x0b => { op_0b }
        0x0c => { op_0c }
        0x0d => { op_0d }
        0x0e => { op_0e }
        0x0f => { op_0f }
        0x10 => { op_10 }
        0x11 => { op_11 }
        0x12 => { op_12 }
        0x13 => { op_13 }
        0x14 => { op_14 }
        0x15 => { op_15 }
        0x16 => { op_16 }
        0x17 => { op_17 }
        0x18 => { op_18 }
        0x19 => { op_19 }
        0x1a => { op_1a }
        0x1b => { op_1b }
        0x1c => { op_1c }
        0x1d => { op_1d }
        0x1e => { op_1e }
        0x1f => { op_1f }
        0x20 => { op_20 }
        0x21 => { op_21 }
        0x22 => { op_22 }
        0x23 => { op_23 }
        0x24 => { op_24 }
        0x25 => { op_25 }
        0x26 => { op_26 }
        0x27 => { op_27 }
        0x28 => { op_28 }
        0x29 => { op_29 }
        0x2a => { op_2a }
        0x2b => { op_2b }
        0x2c => { op_2c }
        0x2d => { op_2d }
        0x2e => { op_2e }
        0x2f => { op_2f }
        0x30 => { op_30 }
        0x31 => { op_31 }
        0x32 => { op_32 }
        0x33 => { op_33 }
        0x34 => { op_34 }
        0x35 => { op_35 }
        0x36 => { op_36 }
        0x37 => { op_37 }
        0x38 => { op_38 }
        0x39 => { op_39 }
        0x3a => { op_3a }
        0x3b => { op_3b }
        0x3c => { op_3c }
        0x3d => { op_3d }
        0x3e => { op_3e }
        0x3f => { op_3f }
        0x40 => { op_40 }
        0x41 => { op_41 }
        0x42 => { op_42 }
        0x43 => { op_43 }
        0x44 => { op_44 }
        0x45 => { op_45 }
        0x46 => { op_46 }
        0x47 => { op_47 }
        0x48 => { op_48 }
        0x49 => { op_49 }
        0x4a => { op_4a }
        0x4b => { op_4b }
        0x4c => { op_4c }
        0x4d => { op_4d }
        0x4e => { op_4e }
        0x4f => { op_4f }
        0x50 => { op_50 }
        0x51 => { op_51 }
        0x52 => { op_52 }
        0x53 => { op_53 }
        0x54 => { op_54 }
        0x55 => { op_55 }
        0x56 => { op_56 }
        0x57 => { op_57 }
        0x58 => { op_58 }
        0x59 => { op_59 }
        0x5a => { op_5a }
        0x5b => { op_5b }
        0x5c => { op_5c }
        0x5d => { op_5d }
        0x5e => { op_5e }
        0x5f => { op_5f }
        0x60 => { op_60 }
        0x61 => { op_61 }
        0x62 => { op_62 }
        0x63 => { op_63 }
        0x64 => { op_64 }
        0x65 => { op_65 }
        0x66 => { op_66 }
        0x67 => { op_67 }
        0x68 => { op_68 }
        0x69 => { op_69 }
        0x6a => { op_6a }
        0x6b => { op_6b }
        0x6c => { op_6c }
        0x6d => { op_6d }
        0x6e => { op_6e }
        0x6f => { op_6f }
        0x70 => { op_70 }
        0x71 => { op_71 }
        0x72 => { op_72 }
        0x73 => { op_73 }
        0x74 => { op_74 }
        0x75 => { op_75 }
        0x76 => { op_76 }
        0x77 => { op_77 }
        0x78 => { op_78 }
        0x79 => { op_79 }
        0x7a => { op_7a }
        0x7b => { op_7b }
        0x7c => { op_7c }
        0x7d => { op_7d }
        0x7e => { op_7e }
        0x7f => { op_7f }
        0x80 => { op_80 }
        0x81 => { op_81 }
        0x82 => { op_82 }
        0x83 => { op_83 }
        0x84 => { op_84 }
        0x85 => { op_85 }
        0x86 => { op_86 }
        0x87 => { op_87 }
        0x88 => { op_88 }
        0x89 => { op_89 }
        0x8a => { op_8a }
        0x8b => { op_8b }
        0x8c => { op_8c }
        0x8d => { op_8d }
        0x8e => { op_8e }
        0x8f => { op_8f }
        0x90 => { op_90 }
        0x91 => { op_91 }
        0x92 => { op_92 }
        0x93 => { op_93 }
        0x94 => { op_94 }
        0x95 => { op_95 }
        0x96 => { op_96 }
        0x97 => { op_97 }
        0x98 => { op_98 }
        0x99 => { op_99 }
        0x9a => { op_9a }
        0x9b => { op_9b }
        0x9c => { op_9c }
        0x9d => { op_9d }
        0x9e => { op_9e }
        0x9f => { op_9f }
        0xa0 => { op_a0 }
        0xa1 => { op_a1 }
        0xa2 => { op_a2 }
        0xa3 => { op_a3 }
        0xa4 => { op_a4 }
        0xa5 => { op_a5 }
        0xa6 => { op_a6 }
        0xa7 => { op_a7 }
        0xa8 => { op_a8 }
        0xa9 => { op_a9 }
        0xaa => { op_aa }
        0xab => { op_ab }
        0xac => { op_ac }
        0xad => { op_ad }
        0xae => { op_ae }
        0xaf => { op_af }
        0xb0 => { op_b0 }
        0xb1 => { op_b1 }
        0xb2 => { op_b2 }
        0xb3 => { op_b3 }
        0xb4 => { op_b4 }
        0xb5 => { op_b5 }
        0xb6 => { op_b6 }
        0xb7 => { op_b7 }
        0xb8 => { op_b8 }
        0xb9 => { op_b9 }
        0xba => { op_ba }
        0xbb => { op_bb }
        0xbc => { op_bc }
        0xbd => { op_bd }
        0xbe => { op_be }
        0xbf => { op_bf }
        0xc0 => { op_c0 }
        0xc1 => { op_c1 }
        0xc2 => { op_c2 }
        0xc3 => { op_c3 }
        0xc4 => { op_c4 }
        0xc5 => { op_c5 }
        0xc6 => { op_c6 }
        0xc7 => { op_c7 }
        0xc8 => { op_c8 }
        0xc9 => { op_c9 }
        0xca => { op_ca }
        0xcb => { op_cb }
        0xcc => { op_cc }
        0xcd => { op_cd }
        0xce => { op_ce }
        0xcf => { op_cf }
        0xd0 => { op_d0 }
        0xd1 => { op_d1 }
        0xd2 => { op_d2 }
        0xd3 => { op_d3 }
        0xd4 => { op_d4 }
        0xd5 => { op_d5 }
        0xd6 => { op_d6 }
        0xd7 => { op_d7 }
        0xd8 => { op_d8 }
        0xd9 => { op_d9 }
        0xda => { op_da }
        0xdb => { op_db }
        0xdc => { op_dc }
        0xdd => { op_dd }
        0xde => { op_de }
        0xdf => { op_df }
        0xe0 => { op_e0 }
        0xe1 => { op_e1 }
        0xe2 => { op_e2 }
        0xe3 => { op_e3 }
        0xe4 => { op_e4 }
        0xe5 => { op_e5 }
        0xe6 => { op_e6 }
        0xe7 => { op_e7 }
        0xe8 => { op_e8 }
        0xe9 => { op_e9 }
        0xea => { op_ea }
        0xeb => { op_eb }
        0xec => { op_ec }
        0xed => { op_ed }
        0xee => { op_ee }
        0xef => { op_ef }
        0xf0 => { op_f0 }
        0xf1 => { op_f1 }
        0xf2 => { op_f2 }
        0xf3 => { op_f3 }
        0xf4 => { op_f4 }
        0xf5 => { op_f5 }
        0xf6 => { op_f6 }
        0xf7 => { op_f7 }
        0xf8 => { op_f8 }
        0xf9 => { op_f9 }
        0xfa => { op_fa }
        0xfb => { op_fb }
        0xfc => { op_fc }
        0xfd => { op_fd }
        0xfe => { op_fe }
        0xff => { op_ff }
        _ => { panic!("opcode not found {}", opcode) }
    }
}

pub fn op_00(cpu: &mut Cpu, _mmu: &mut Mmu) {
    println!("NOP");
    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_01(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.set_bc(mmu.read_word(cpu.pc));
    cpu.pc += 2;
    cpu.cycles += 3;
}

pub fn op_02(cpu: &mut Cpu, mmu: &mut Mmu) {
    mmu.write_byte(cpu.get_bc(), cpu.a);
    cpu.pc += 1;
    cpu.cycles += 7
}

pub fn op_03(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.set_bc(cpu.get_bc().wrapping_add(1));
    cpu.pc += 1;
    cpu.cycles += 6;
}

pub fn op_04(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.b = cpu.apply_inc8_with_flags(cpu.b);
    cpu.pc += 1;
    cpu.cycles += 4;
}

pub fn op_05(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.b = cpu.apply_dec8_with_flags(cpu.b);
    cpu.pc += 1;
    cpu.cycles += 4;
}

pub fn op_06(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.b = mmu.read_byte(cpu.pc + 1);
    cpu.pc += 2;
    cpu.cycles += 7;
}

pub fn op_07(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_rotate_left_with_flags(cpu.a, false);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_08(cpu: &mut Cpu, mmu: &mut Mmu) {
    let address = mmu.read_word(cpu.pc + 1);
    mmu.write_word(address, cpu.sp);

    cpu.pc += 3;
    cpu.cycles += 5;
}

pub fn op_09(cpu: &mut Cpu, _mmu: &mut Mmu) {
    let value = cpu.apply_add16_with_flags(cpu.get_hl(), cpu.get_bc());
    cpu.set_hl(value);

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_0a(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.a = mmu.read_byte(cpu.get_bc());

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_0b(cpu: &mut Cpu, _mmu: &mut Mmu) {
    let value: u16 = cpu.get_bc().wrapping_add(1);
    cpu.set_bc(value);

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_0c(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.c = cpu.apply_inc8_with_flags(cpu.c);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_0d(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.c = cpu.apply_dec8_with_flags(cpu.c);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_0e(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.c = mmu.read_byte(cpu.pc + 1);

    cpu.pc += 2;
    cpu.cycles += 2;
}

pub fn op_0f(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_rotate_right_with_flags(cpu.a, false);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_10(cpu: &mut Cpu, mmu: &mut Mmu) {
    mmu.toggle_speed();

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_11(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.set_de(mmu.read_word(cpu.pc + 1));

    cpu.pc += 3;
    cpu.cycles += 3;
}

pub fn op_12(cpu: &mut Cpu, mmu: &mut Mmu) {
    mmu.write_byte(cpu.get_de(), cpu.a);

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_13(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.set_de(cpu.get_de().wrapping_add(1));

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_14(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.d = cpu.apply_inc8_with_flags(cpu.d);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_15(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.d = cpu.apply_dec8_with_flags(cpu.d);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_16(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.d = mmu.read_byte(cpu.pc + 1);

    cpu.pc += 2;
    cpu.cycles += 2;
}

pub fn op_17(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_rotate_left_with_flags(cpu.a, true);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_18(cpu: &mut Cpu, mmu: &mut Mmu) {
    // TODO
    mmu.read_byte(cpu.pc + 1);
    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_19(cpu: &mut Cpu, _mmu: &mut Mmu) {
    let value = cpu.apply_add16_with_flags(cpu.get_hl(), cpu.get_de());
    cpu.set_hl(value);

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_1a(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.a = mmu.read_byte(cpu.get_de());

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_1b(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.set_de(cpu.get_de().wrapping_sub(1));

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_1c(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.e = cpu.apply_inc8_with_flags(cpu.e);

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_1d(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.e = cpu.apply_dec8_with_flags(cpu.e);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_1e(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.e = mmu.read_byte(cpu.pc + 1);

    cpu.pc += 2;
    cpu.cycles += 2;
}

pub fn op_1f(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_rotate_right_with_flags(cpu.a, true);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_20(cpu: &mut Cpu, mmu: &mut Mmu) {
    // TODO
    mmu.read_byte(cpu.pc + 1);
    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_21(cpu: &mut Cpu, mmu: &mut Mmu) {
    let value = mmu.read_word(cpu.pc + 1);
    cpu.set_hl(value);

    cpu.pc += 2;
    cpu.cycles += 3;
}

pub fn op_22(cpu: &mut Cpu, mmu: &mut Mmu) {
    let addr = cpu.get_hl().wrapping_add(1);
    mmu.write_byte(addr, cpu.a);
    cpu.set_hl(addr);

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_23(cpu: &mut Cpu, _mmu: &mut Mmu) {
    let value = cpu.get_hl().wrapping_add(1);
    cpu.set_hl(value);

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_24(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.h = cpu.apply_inc8_with_flags(cpu.h);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_25(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.h = cpu.apply_dec8_with_flags(cpu.h);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_26(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.h = mmu.read_byte(cpu.pc + 1);

    cpu.pc += 2;
    cpu.cycles += 2;
}

pub fn op_27(cpu: &mut Cpu, mmu: &mut Mmu) {
    // TODO
    mmu.read_byte(cpu.pc + 1);
    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_28(cpu: &mut Cpu, mmu: &mut Mmu) {
    // TODO
    mmu.read_byte(cpu.pc + 1);
    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_29(cpu: &mut Cpu, _mmu: &mut Mmu) {
    let result = cpu.apply_add16_with_flags(cpu.get_hl(), cpu.sp);
    cpu.set_hl(result);

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_2a(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.a = mmu.read_byte(cpu.get_hl().wrapping_add(1));

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_2b(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.set_hl(cpu.get_hl().wrapping_sub(1));


    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_2c(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.l = cpu.apply_inc8_with_flags(cpu.l);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_2d(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.l = cpu.apply_dec8_with_flags(cpu.l);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_2e(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.l = mmu.read_byte(cpu.pc + 1);

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_2f(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = !cpu.a;
    cpu.set_f_half_carry(true);
    cpu.set_f_negative(true);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_30(cpu: &mut Cpu, _mmu: &mut Mmu) {
    // TODO

    cpu.pc += 2;
    cpu.cycles += 3;
}

pub fn op_31(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.sp = mmu.read_word(cpu.pc + 1);
    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_32(cpu: &mut Cpu, mmu: &mut Mmu) {
    mmu.write_byte(cpu.get_hl(), cpu.a);
    cpu.set_hl(cpu.get_hl() - 1);

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_33(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.sp = cpu.sp.wrapping_add(1);

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_34(cpu: &mut Cpu, mmu: &mut Mmu) {
    let addr = cpu.get_hl();
    let mut value = mmu.read_byte(addr);
    value = cpu.apply_inc8_with_flags(value);
    mmu.write_byte(cpu.a as u16, value);

    cpu.pc += 1;
    cpu.cycles += 3;
}

pub fn op_35(cpu: &mut Cpu, mmu: &mut Mmu) {
    let addr = cpu.get_hl();
    let mut value = mmu.read_byte(addr);
    value = cpu.apply_dec8_with_flags(value);
    mmu.write_byte(cpu.a as u16, value);

    cpu.pc += 1;
    cpu.cycles += 3;
}

pub fn op_36(cpu: &mut Cpu, mmu: &mut Mmu) {
    let value = mmu.read_byte(cpu.pc + 1);
    mmu.write_byte(cpu.get_hl(), value);

    cpu.pc += 2;
    cpu.cycles += 3;
}

pub fn op_37(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.set_f_carry(true);
    cpu.set_f_half_carry(false);
    cpu.set_f_negative(false);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_38(cpu: &mut Cpu, _mmu: &mut Mmu) {
    // TODO

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_39(cpu: &mut Cpu, _mmu: &mut Mmu) {
    let hl = cpu.get_hl();
    let sp = cpu.sp;

    let result = cpu.apply_add16_with_flags(hl, sp);
    cpu.set_hl(result);

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_3a(cpu: &mut Cpu, mmu: &mut Mmu) {
    let hl = cpu.get_hl();
    cpu.a = mmu.read_byte(hl);
    cpu.set_hl(hl - 1);

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_3b(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.sp = cpu.sp.wrapping_sub(1);

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_3c(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_inc8_with_flags(cpu.a);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_3d(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_dec8_with_flags(cpu.a);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_3e(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.a = mmu.read_byte(cpu.pc + 1);

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_3f(cpu: &mut Cpu, mmu: &mut Mmu) {
    // TODO
    mmu.read_byte(cpu.pc + 1);
    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_40(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_41(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.b = cpu.c;

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_42(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.b = cpu.d;

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_43(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.b = cpu.e;

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_44(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.b = cpu.h;

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_45(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.b = cpu.l;

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_46(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.b = mmu.read_byte(cpu.get_hl());

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_47(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.b = cpu.a;

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_48(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.c = cpu.b;

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_49(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_4a(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.c = cpu.d;

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_4b(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.c = cpu.e;

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_4c(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.c = cpu.h;

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_4d(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.c = cpu.l;

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_4e(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.c = mmu.read_byte(cpu.get_hl());

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_4f(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.c = cpu.a;

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_50(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.d = cpu.b;

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_51(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.d = cpu.c;

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_52(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_53(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.d = cpu.e;

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_54(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.d = cpu.h;

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_55(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.d = cpu.l;

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_56(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.d = mmu.read_byte(cpu.get_hl());

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_57(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.d = cpu.a;

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_58(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.e = cpu.b;

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_59(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.e = cpu.c;

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_5a(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.e = cpu.d;

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_5b(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_5c(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.e = cpu.h;

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_5d(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.e = cpu.l;

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_5e(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.e = mmu.read_byte(cpu.get_hl());

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_5f(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.e = cpu.a;

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_60(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.h = cpu.b;

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_61(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.h = cpu.c;

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_62(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.h = cpu.d;

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_63(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.h = cpu.e;

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_64(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_65(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.h = cpu.l;

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_66(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.h = mmu.read_byte(cpu.get_hl());

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_67(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.h = cpu.a;

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_68(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.l = cpu.b;

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_69(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.l = cpu.c;

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_6a(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.l = cpu.d;

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_6b(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.l = cpu.e;

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_6c(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.l = cpu.h;

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_6d(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_6e(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.l = mmu.read_byte(cpu.get_hl());

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_6f(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.l = cpu.a;

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_70(cpu: &mut Cpu, mmu: &mut Mmu) {
    mmu.write_byte(cpu.get_hl(), cpu.b);

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_71(cpu: &mut Cpu, mmu: &mut Mmu) {
    mmu.write_byte(cpu.get_hl(), cpu.c);

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_72(cpu: &mut Cpu, mmu: &mut Mmu) {
    mmu.write_byte(cpu.get_hl(), cpu.d);

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_73(cpu: &mut Cpu, mmu: &mut Mmu) {
    mmu.write_byte(cpu.get_hl(), cpu.e);

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_74(cpu: &mut Cpu, mmu: &mut Mmu) {
    mmu.write_byte(cpu.get_hl(), cpu.h);

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_75(cpu: &mut Cpu, mmu: &mut Mmu) {
    mmu.write_byte(cpu.get_hl(), cpu.h);

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_76(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.halted = true;

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_77(cpu: &mut Cpu, mmu: &mut Mmu) {
    mmu.write_byte(cpu.get_hl(), cpu.a);

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_78(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.b;

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_79(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.c;

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_7a(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.d;

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_7b(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.e;

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_7c(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.h;

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_7d(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.l;

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_7e(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.a = mmu.read_byte(cpu.get_hl());

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_7f(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_80(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_add8_with_flags(cpu.a, cpu.b, false);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_81(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_add8_with_flags(cpu.a, cpu.c, false);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_82(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_add8_with_flags(cpu.a, cpu.d, false);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_83(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_add8_with_flags(cpu.a, cpu.e, false);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_84(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_add8_with_flags(cpu.a, cpu.h, false);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_85(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_add8_with_flags(cpu.a, cpu.l, false);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_86(cpu: &mut Cpu, mmu: &mut Mmu) {
    let value = mmu.read_byte(cpu.get_hl());
    cpu.a = cpu.apply_add8_with_flags(cpu.a, value, false);

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_87(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_add8_with_flags(cpu.a, cpu.a, false);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_88(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_add8_with_flags(cpu.a, cpu.b, true);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_89(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_add8_with_flags(cpu.a, cpu.c, true);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_8a(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_add8_with_flags(cpu.a, cpu.d, true);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_8b(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_add8_with_flags(cpu.a, cpu.e, true);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_8c(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_add8_with_flags(cpu.a, cpu.h, true);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_8d(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_add8_with_flags(cpu.a, cpu.l, true);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_8e(cpu: &mut Cpu, mmu: &mut Mmu) {
    let value = mmu.read_byte(cpu.get_hl());
    cpu.a = cpu.apply_add8_with_flags(cpu.a, value, true);

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_8f(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_add8_with_flags(cpu.a, cpu.a, true);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_90(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_sub8_with_flags(cpu.a, cpu.h, false);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_91(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_sub8_with_flags(cpu.a, cpu.c, false);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_92(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_sub8_with_flags(cpu.a, cpu.d, false);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_93(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_sub8_with_flags(cpu.a, cpu.d, false);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_94(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_sub8_with_flags(cpu.a, cpu.h, false);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_95(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_sub8_with_flags(cpu.a, cpu.l, false);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_96(cpu: &mut Cpu, mmu: &mut Mmu) {
    let value = mmu.read_byte(cpu.get_hl());
    cpu.a = cpu.apply_sub8_with_flags(cpu.a, value, false);

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_97(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_sub8_with_flags(cpu.a, cpu.a, false);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_98(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_sub8_with_flags(cpu.a, cpu.b, true);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_99(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_sub8_with_flags(cpu.a, cpu.c, true);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_9a(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_sub8_with_flags(cpu.a, cpu.d, true);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_9b(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_sub8_with_flags(cpu.a, cpu.e, true);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_9c(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_sub8_with_flags(cpu.a, cpu.h, true);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_9d(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_sub8_with_flags(cpu.a, cpu.l, true);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_9e(cpu: &mut Cpu, mmu: &mut Mmu) {
    let value = mmu.read_byte(cpu.get_hl());
    cpu.a = cpu.apply_sub8_with_flags(cpu.a, value, true);

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_9f(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_sub8_with_flags(cpu.a, cpu.a, true);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_a0(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_and8_with_flags(cpu.a, cpu.b);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_a1(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_and8_with_flags(cpu.a, cpu.c);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_a2(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_and8_with_flags(cpu.a, cpu.d);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_a3(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_and8_with_flags(cpu.a, cpu.e);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_a4(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_and8_with_flags(cpu.a, cpu.h);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_a5(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_and8_with_flags(cpu.a, cpu.l);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_a6(cpu: &mut Cpu, mmu: &mut Mmu) {
    let value = mmu.read_byte(cpu.get_hl());
    cpu.a = cpu.apply_and8_with_flags(cpu.a, value);

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_a7(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_and8_with_flags(cpu.a, cpu.a);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_a8(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_xor8_with_flags(cpu.a, cpu.b);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_a9(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_xor8_with_flags(cpu.a, cpu.c);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_aa(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_xor8_with_flags(cpu.a, cpu.d);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_ab(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_xor8_with_flags(cpu.a, cpu.e);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_ac(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_xor8_with_flags(cpu.a, cpu.h);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_ad(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_xor8_with_flags(cpu.a, cpu.l);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_ae(cpu: &mut Cpu, mmu: &mut Mmu) {
    let value = mmu.read_byte(cpu.get_hl());
    cpu.a = cpu.apply_xor8_with_flags(cpu.a, value);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_af(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_xor8_with_flags(cpu.a, cpu.a);

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_b0(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_or8_with_flags(cpu.a, cpu.b);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_b1(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_or8_with_flags(cpu.a, cpu.c);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_b2(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_or8_with_flags(cpu.a, cpu.d);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_b3(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_or8_with_flags(cpu.a, cpu.e);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_b4(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_or8_with_flags(cpu.a, cpu.h);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_b5(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_or8_with_flags(cpu.a, cpu.l);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_b6(cpu: &mut Cpu, mmu: &mut Mmu) {
    let value = mmu.read_byte(cpu.get_hl());
    cpu.a = cpu.apply_or8_with_flags(cpu.a, value);

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_b7(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.a = cpu.apply_or8_with_flags(cpu.a, cpu.a);

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_b8(cpu: &mut Cpu, _mmu: &mut Mmu) {
    let a = cpu.a;
    cpu.apply_sub8_with_flags(cpu.a, cpu.b, false);
    cpu.a = a;

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_b9(cpu: &mut Cpu, _mmu: &mut Mmu) {
    let a = cpu.a;
    cpu.apply_sub8_with_flags(cpu.a, cpu.c, false);
    cpu.a = a;

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_ba(cpu: &mut Cpu, _mmu: &mut Mmu) {
    let a = cpu.a;
    cpu.apply_sub8_with_flags(cpu.a, cpu.d, false);
    cpu.a = a;

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_bb(cpu: &mut Cpu, _mmu: &mut Mmu) {
    let a = cpu.a;
    cpu.apply_sub8_with_flags(cpu.a, cpu.e, false);
    cpu.a = a;

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_bc(cpu: &mut Cpu, _mmu: &mut Mmu) {
    let a = cpu.a;
    cpu.apply_sub8_with_flags(cpu.a, cpu.h, false);
    cpu.a = a;

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_bd(cpu: &mut Cpu, _mmu: &mut Mmu) {
    let a = cpu.a;
    cpu.apply_sub8_with_flags(cpu.a, cpu.l, false);
    cpu.a = a;

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_be(cpu: &mut Cpu, mmu: &mut Mmu) {
    let value = mmu.read_byte(cpu.get_hl());
    let a = cpu.a;
    cpu.apply_sub8_with_flags(cpu.a, value, false);
    cpu.a = a;

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_bf(cpu: &mut Cpu, _mmu: &mut Mmu) {
    let a = cpu.a;
    cpu.apply_sub8_with_flags(cpu.a, cpu.a, false);
    cpu.a = a;

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_c0(cpu: &mut Cpu, mmu: &mut Mmu) {
    if !cpu.get_f_zero() {
        cpu.pc = cpu.pop_word(mmu);
        cpu.cycles += 5;
    } else {
        cpu.pc += 2;
    }
}

pub fn op_c1(cpu: &mut Cpu, mmu: &mut Mmu) {
    let value = cpu.pop_word(mmu);
    cpu.set_bc(value);

    cpu.pc += 1;
    cpu.cycles += 3;
}

pub fn op_c2(cpu: &mut Cpu, mmu: &mut Mmu) {
    if !cpu.get_f_zero() {
        cpu.pc = mmu.read_word(cpu.pc + 1);

        cpu.cycles += 4;
    } else {
        cpu.pc += 2;

        cpu.cycles += 3;
    }
}

pub fn op_c3(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.pc = mmu.read_word(cpu.pc + 1);

    cpu.cycles += 4;
}

pub fn op_c4(cpu: &mut Cpu, mmu: &mut Mmu) {
    if !cpu.get_f_zero() {
        cpu.push_word(mmu, cpu.pc + 2);
        cpu.pc = mmu.read_word(cpu.pc + 1);

        cpu.cycles += 6;
    } else {
        cpu.pc += 2;

        cpu.cycles += 3;
    }
}

pub fn op_c5(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.push_word(mmu, cpu.get_bc());

    cpu.pc += 1;
    cpu.cycles += 4;
}

pub fn op_c6(cpu: &mut Cpu, mmu: &mut Mmu) {
    let value = mmu.read_byte(cpu.pc + 1);
    cpu.a = cpu.apply_add8_with_flags(cpu.a, value, false);


    cpu.pc += 2;
    cpu.cycles += 2;
}

pub fn op_c7(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.push_word(mmu, cpu.pc);
    cpu.pc = 0x00;

    cpu.pc += 1;
    cpu.cycles += 4;
}

pub fn op_c8(cpu: &mut Cpu, mmu: &mut Mmu) {
    if !cpu.get_f_zero() {
        cpu.pc = cpu.pop_word(mmu);

        cpu.cycles += 5;
    } else {
        cpu.pc += 1;
        cpu.cycles += 2;
    }
}

pub fn op_c9(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.pc = cpu.pop_word(mmu);
    cpu.cycles += 4;
}

pub fn op_ca(cpu: &mut Cpu, mmu: &mut Mmu) {
    if cpu.get_f_zero() {
        cpu.pc = mmu.read_word(cpu.pc + 1);

        cpu.cycles += 4;
    } else {
        cpu.pc += 2;
        cpu.cycles += 3;
    }
}

pub fn op_cb(cpu: &mut Cpu, mmu: &mut Mmu) {
    let opcode = mmu.read_byte(cpu.pc + 1);

    cpu.pc += 1;
    match opcode {
        0x00 => {
            cpu.b = cpu.apply_rotate_left_with_flags(cpu.b, false);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x01 => {
            cpu.c = cpu.apply_rotate_left_with_flags(cpu.c, false);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x02 => {
            cpu.d = cpu.apply_rotate_left_with_flags(cpu.d, false);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x03 => {
            cpu.e = cpu.apply_rotate_left_with_flags(cpu.e, false);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x04 => {
            cpu.h = cpu.apply_rotate_left_with_flags(cpu.h, false);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x05 => {
            cpu.l = cpu.apply_rotate_left_with_flags(cpu.l, false);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x06 => {
            let address = cpu.get_hl();
            let value = mmu.read_byte(address);

            let result = cpu.apply_rotate_left_with_flags(value, false);
            mmu.write_byte(address, result);

            cpu.pc += 2;
            cpu.cycles += 4;
        }
        0x07 => {
            cpu.a = cpu.apply_rotate_left_with_flags(cpu.a, false);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x08 => {
            cpu.a = cpu.apply_rotate_right_with_flags(cpu.a, false);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x09 => {
            cpu.b = cpu.apply_rotate_right_with_flags(cpu.b, false);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x0A => {
            cpu.d = cpu.apply_rotate_right_with_flags(cpu.d, false);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x0B => {
            cpu.e = cpu.apply_rotate_right_with_flags(cpu.e, false);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x0C => {
            cpu.h = cpu.apply_rotate_right_with_flags(cpu.h, false);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x0D => {
            cpu.l = cpu.apply_rotate_right_with_flags(cpu.l, false);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x0E => {
            let address = cpu.get_hl();
            let value = mmu.read_byte(address);

            let result = cpu.apply_rotate_right_with_flags(value, false);
            mmu.write_byte(address, result);

            cpu.pc += 2;
            cpu.cycles += 4;
        }
        0x0F => {
            cpu.a = cpu.apply_rotate_right_with_flags(cpu.a, false);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x10 => {
            cpu.b = cpu.apply_rotate_left_with_flags(cpu.b, true);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x11 => {
            cpu.c = cpu.apply_rotate_left_with_flags(cpu.c, true);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x12 => {
            cpu.d = cpu.apply_rotate_left_with_flags(cpu.d, true);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x13 => {
            cpu.e = cpu.apply_rotate_left_with_flags(cpu.e, true);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x14 => {
            cpu.h = cpu.apply_rotate_left_with_flags(cpu.h, true);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x15 => {
            cpu.l = cpu.apply_rotate_left_with_flags(cpu.l, true);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x16 => {
            let address = cpu.get_hl();
            let value = mmu.read_byte(address);

            let result = cpu.apply_rotate_left_with_flags(value, true);
            mmu.write_byte(address, result);

            cpu.pc += 2;
            cpu.cycles += 4;
        }
        0x17 => {
            cpu.a = cpu.apply_rotate_left_with_flags(cpu.a, true);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x18 => {
            cpu.b = cpu.apply_rotate_right_with_flags(cpu.b, true);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x19 => {
            cpu.c = cpu.apply_rotate_right_with_flags(cpu.c, true);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x1A => {
            cpu.d = cpu.apply_rotate_right_with_flags(cpu.d, true);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x1B => {
            cpu.e = cpu.apply_rotate_right_with_flags(cpu.e, true);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x1C => {
            cpu.h = cpu.apply_rotate_right_with_flags(cpu.h, true);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x1D => {
            cpu.l = cpu.apply_rotate_right_with_flags(cpu.l, true);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x1E => {
            let address = cpu.get_hl();
            let value = mmu.read_byte(address);

            let result = cpu.apply_rotate_right_with_flags(value, true);
            mmu.write_byte(address, result);

            cpu.pc += 2;
            cpu.cycles += 4;
        }
        0x1F => {
            cpu.a = cpu.apply_rotate_right_with_flags(cpu.a, true);
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
            cpu.c = cpu.apply_shift_left_with_flags(cpu.c);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x23 => {
            cpu.d = cpu.apply_shift_left_with_flags(cpu.d);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x24 => {
            cpu.e = cpu.apply_shift_left_with_flags(cpu.e);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x25 => {
            cpu.h = cpu.apply_shift_left_with_flags(cpu.h);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x26 => {
            cpu.l = cpu.apply_shift_left_with_flags(cpu.l);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x27 => {
            let address = cpu.get_hl();
            let value = mmu.read_byte(address);

            let result = cpu.apply_shift_left_with_flags(value);
            mmu.write_byte(address, result);

            cpu.pc += 2;
            cpu.cycles += 4;
        }
        0x28 => {
            cpu.a = cpu.apply_shift_left_with_flags(cpu.a);
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

            cpu.pc += 2;
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

            cpu.pc += 2;
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

            cpu.pc += 2;
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

            cpu.pc += 2;
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

            cpu.pc += 2;
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

            cpu.pc += 2;
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

            cpu.pc += 2;
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

            cpu.pc += 2;
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

            cpu.pc += 2;
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

            cpu.pc += 2;
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

            cpu.pc += 2;
            cpu.cycles += 4;
        }
        0x7F => {
            cpu.apply_bit_test(cpu.a, 7);
            cpu.pc += 1;
            cpu.cycles += 2;
        }
        0x80 => {}
        0x81 => {}
        0x82 => {}
        0x83 => {}
        0x84 => {}
        0x85 => {}
        0x86 => {}
        0x87 => {}
        0x88 => {}
        0x89 => {}
        0x8A => {}
        0x8B => {}
        0x8C => {}
        0x8D => {}
        0x8E => {}
        0x8F => {}
        0x90 => {}
        0x91 => {}
        0x92 => {}
        0x93 => {}
        0x94 => {}
        0x95 => {}
        0x96 => {}
        0x97 => {}
        0x98 => {}
        0x99 => {}
        0x9A => {}
        0x9B => {}
        0x9C => {}
        0x9D => {}
        0x9E => {}
        0x9F => {}
        0xA0 => {}
        0xA1 => {}
        0xA2 => {}
        0xA3 => {}
        0xA4 => {}
        0xA5 => {}
        0xA6 => {}
        0xA7 => {}
        0xA8 => {}
        0xA9 => {}
        0xAA => {}
        0xAB => {}
        0xAC => {}
        0xAD => {}
        0xAE => {}
        0xAF => {}
        0xB0 => {}
        0xB1 => {}
        0xB2 => {}
        0xB3 => {}
        0xB4 => {}
        0xB5 => {}
        0xB6 => {}
        0xB7 => {}
        0xB8 => {}
        0xB9 => {}
        0xBA => {}
        0xBB => {}
        0xBC => {}
        0xBD => {}
        0xBE => {}
        0xBF => {}
        0xC0 => {}
        0xC1 => {}
        0xC2 => {}
        0xC3 => {}
        0xC4 => {}
        0xC5 => {}
        0xC6 => {}
        0xC7 => {}
        0xC8 => {}
        0xC9 => {}
        0xCA => {}
        0xCB => {}
        0xCC => {}
        0xCD => {}
        0xCE => {}
        0xCF => {}
        0xD0 => {}
        0xD1 => {}
        0xD2 => {}
        0xD3 => {}
        0xD4 => {}
        0xD5 => {}
        0xD6 => {}
        0xD7 => {}
        0xD8 => {}
        0xD9 => {}
        0xDA => {}
        0xDB => {}
        0xDC => {}
        0xDD => {}
        0xDE => {}
        0xDF => {}
        0xE0 => {}
        0xE1 => {}
        0xE2 => {}
        0xE3 => {}
        0xE4 => {}
        0xE5 => {}
        0xE6 => {}
        0xE7 => {}
        0xE8 => {}
        0xE9 => {}
        0xEA => {}
        0xEB => {}
        0xEC => {}
        0xED => {}
        0xEE => {}
        0xEF => {}
        0xF0 => {}
        0xF1 => {}
        0xF2 => {}
        0xF3 => {}
        0xF4 => {}
        0xF5 => {}
        0xF6 => {}
        0xF7 => {}
        0xF8 => {}
        0xF9 => {}
        0xFA => {}
        0xFB => {}
        0xFC => {}
        0xFD => {}
        0xFE => {}
        0xFF => {}
        _ => { panic!("opcode not found {}", opcode) }
    }

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_cc(cpu: &mut Cpu, mmu: &mut Mmu) {
    if cpu.get_f_zero() {
        cpu.push_word(mmu,cpu.pc + 2);
        cpu.pc = mmu.read_word(cpu.pc + 1);

        cpu.cycles += 6;
    } else {
        cpu.pc += 2;
        cpu.cycles += 3;
    }
}

pub fn op_cd(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.push_word(mmu, cpu.pc + 2);
    cpu.pc = mmu.read_word(cpu.pc + 1);

    cpu.cycles += 6;
}

pub fn op_ce(cpu: &mut Cpu, mmu: &mut Mmu) {
    let value = mmu.read_byte(cpu.pc + 1);
    cpu.a = cpu.apply_add8_with_flags(cpu.a, value, true);

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_cf(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.push_word(mmu, cpu.pc);
    cpu.pc = 0x08;

    cpu.cycles += 4;
}

pub fn op_d0(cpu: &mut Cpu, mmu: &mut Mmu) {
    if !cpu.get_f_carry() {
        cpu.pc = cpu.pop_word(mmu);

        cpu.cycles += 5;
    } else {
        cpu.cycles += 2;
    }
}

pub fn op_d1(cpu: &mut Cpu, mmu: &mut Mmu) {
    let value = cpu.pop_word(mmu);
    cpu.set_de(value);

    cpu.pc += 1;
    cpu.cycles += 3;
}

pub fn op_d2(cpu: &mut Cpu, mmu: &mut Mmu) {
    if !cpu.get_f_carry() {
        cpu.pc = mmu.read_word(cpu.pc + 1);
        cpu.cycles += 4;
    } else {
        cpu.pc += 2;
        cpu.cycles += 3;
    }
}

pub fn op_d3(_cpu: &mut Cpu, _mmu: &mut Mmu) {
    panic!("error");
}

pub fn op_d4(cpu: &mut Cpu, mmu: &mut Mmu) {
    if !cpu.get_f_carry() {
        cpu.push_word(mmu, cpu.pc + 2);
        cpu.pc = mmu.read_word(cpu.pc + 1);
        cpu.cycles += 6;
    } else {
        cpu.pc += 2;
        cpu.cycles += 3;
    }
}

pub fn op_d5(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.push_word(mmu, cpu.get_de());

    cpu.pc += 1;
    cpu.cycles += 4;
}

pub fn op_d6(cpu: &mut Cpu, mmu: &mut Mmu) {
    let value = mmu.read_byte(cpu.pc + 1);
    cpu.a = cpu.apply_sub8_with_flags(cpu.a, value, false);

    cpu.pc += 2;
    cpu.cycles += 2;
}

pub fn op_d7(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.push_word(mmu, cpu.pc);
    cpu.pc = 0x10;

    cpu.pc += 1;
    cpu.cycles += 4;
}

pub fn op_d8(cpu: &mut Cpu, mmu: &mut Mmu) {
    if cpu.get_f_carry() {
        cpu.pc = cpu.pop_word(mmu);

        cpu.cycles += 5;
    } else {
        cpu.pc += 1;
        cpu.cycles += 2;
    }
}

pub fn op_d9(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.pc = cpu.pop_word(mmu);
    cpu.interrupt_enable = true;

    cpu.pc += 1;
    cpu.cycles += 4;
}

pub fn op_da(cpu: &mut Cpu, mmu: &mut Mmu) {
    if cpu.get_f_carry() {
        cpu.pc = mmu.read_word(cpu.pc + 1);

        cpu.cycles += 4;
    } else {

        cpu.pc += 2;
        cpu.cycles += 3;
    }
}

pub fn op_db(_cpu: &mut Cpu, _mmu: &mut Mmu) {
    panic!("error");
}

pub fn op_dc(cpu: &mut Cpu, mmu: &mut Mmu) {
    if cpu.get_f_carry() {
        cpu.push_word(mmu, cpu.pc + 2);
        cpu.pc = mmu.read_word(cpu.pc + 1);
        cpu.cycles += 6;
    } else {
        cpu.pc += 2;
        cpu.cycles += 3;
    }
}

pub fn op_dd(_cpu: &mut Cpu, _mmu: &mut Mmu) {
    panic!("error")
}

pub fn op_de(cpu: &mut Cpu, mmu: &mut Mmu) {
    let value = mmu.read_byte(cpu.pc + 1);
    cpu.a = cpu.apply_sub8_with_flags(cpu.a, value, true);

    cpu.pc += 2;
    cpu.cycles += 2;
}

pub fn op_df(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.push_word(mmu, cpu.pc);
    cpu.pc = 0x18;

    cpu.pc += 1;
    cpu.cycles += 4;
}

pub fn op_e0(cpu: &mut Cpu, mmu: &mut Mmu) {
    let value = mmu.read_byte(cpu.pc + 1) as u16;
    let a = 0xFF00 | value;

    mmu.write_byte(a, cpu.a);

    cpu.pc += 2;
    cpu.cycles += 3;
}

pub fn op_e1(cpu: &mut Cpu, mmu: &mut Mmu) {
    let value = cpu.pop_word(mmu);
    cpu.set_hl(value);

    cpu.pc += 1;
    cpu.cycles += 3;
}

pub fn op_e2(cpu: &mut Cpu, mmu: &mut Mmu) {
    let address = 0xFF00 | (cpu.c as u16);
    mmu.write_byte(address, cpu.a);

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_e3(_cpu: &mut Cpu, _mmu: &mut Mmu) {
    panic!("error")
}

pub fn op_e4(_cpu: &mut Cpu, _mmu: &mut Mmu) {
    panic!("error")
}

pub fn op_e5(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.push_word(mmu, cpu.get_hl());

    cpu.pc += 1;
    cpu.cycles += 4;
}

pub fn op_e6(cpu: &mut Cpu, mmu: &mut Mmu) {
    let value = mmu.read_byte(cpu.pc + 1);
    cpu.a = cpu.apply_and8_with_flags(cpu.a, value);

    cpu.pc += 2;
    cpu.cycles += 2;
}

pub fn op_e7(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.push_word(mmu, cpu.pc);
    cpu.pc = 0x20;

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_e8(cpu: &mut Cpu, mmu: &mut Mmu) {
    // TODO
    mmu.read_byte(cpu.pc + 1);
    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_e9(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.pc = cpu.get_hl();

    cpu.pc += 1;
    cpu.cycles += 1;
}

pub fn op_ea(cpu: &mut Cpu, mmu: &mut Mmu) {
    let a = mmu.read_word(cpu.pc + 1);
    mmu.write_byte(a, cpu.a);

    cpu.pc += 3;
    cpu.cycles += 4;
}

pub fn op_eb(_cpu: &mut Cpu, _mmu: &mut Mmu) {
    panic!("error");
}

pub fn op_ec(_cpu: &mut Cpu, _mmu: &mut Mmu) {
    panic!("error");
}

pub fn op_ed(_cpu: &mut Cpu, _mmu: &mut Mmu) {
    panic!("error");
}

pub fn op_ee(cpu: &mut Cpu, mmu: &mut Mmu) {
    let value = mmu.read_byte(cpu.pc + 1);
    cpu.a = cpu.apply_xor8_with_flags(cpu.a, value);

    cpu.pc += 2;
    cpu.cycles += 2;
}

pub fn op_ef(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.push_word(mmu, cpu.pc);
    cpu.pc = 0x28;

    cpu.pc += 1;
    cpu.cycles += 4;
}

pub fn op_f0(cpu: &mut Cpu, mmu: &mut Mmu) {
    let address = 0xFF00 | mmu.read_byte(cpu.pc + 1) as u16;
    cpu.a = mmu.read_byte(address);

    cpu.pc += 2;
    cpu.cycles += 3;
}

pub fn op_f1(cpu: &mut Cpu, mmu: &mut Mmu) {
    let value = cpu.pop_word(mmu) & 0xFFF0;
    cpu.set_af(value);

    cpu.pc += 1;
    cpu.cycles += 3;
}

pub fn op_f2(cpu: &mut Cpu, mmu: &mut Mmu) {
    let address = 0xFF00 | cpu.c as u16;
    cpu.a = mmu.read_byte(address);

    cpu.pc += 1;
    cpu.cycles += 3;
}

pub fn op_f3(cpu: &mut Cpu, mmu: &mut Mmu) {
    // TODO
    mmu.read_byte(cpu.pc + 1);
    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_f4(_cpu: &mut Cpu, _mmu: &mut Mmu) {
    panic!("error");
}

pub fn op_f5(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.push_word(mmu, cpu.get_af());

    cpu.pc += 1;
    cpu.cycles += 4;
}

pub fn op_f6(cpu: &mut Cpu, mmu: &mut Mmu) {
    let value = mmu.read_byte(cpu.pc + 1);
    cpu.a = cpu.apply_or8_with_flags(cpu.a, value);

    cpu.pc += 2;
    cpu.cycles += 2;
}

pub fn op_f7(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.push_word(mmu,cpu.pc);
    cpu.pc = 0x30;

    cpu.pc += 1;
    cpu.cycles += 4;
}

pub fn op_f8(cpu: &mut Cpu, _mmu: &mut Mmu) {
    // TODO
    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_f9(cpu: &mut Cpu, _mmu: &mut Mmu) {
    cpu.sp = cpu.get_hl();

    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_fa(cpu: &mut Cpu, mmu: &mut Mmu) {
    let address = mmu.read_word(cpu.pc + 1);
    cpu.a = mmu.read_byte(address);

    cpu.pc += 3;
    cpu.cycles += 4;
}

pub fn op_fb(cpu: &mut Cpu, mmu: &mut Mmu) {
    // TODO
    mmu.read_byte(cpu.pc + 1);
    cpu.pc += 1;
    cpu.cycles += 2;
}

pub fn op_fc(_cpu: &mut Cpu, _mmu: &mut Mmu) {
    panic!("error");
}

pub fn op_fd(_cpu: &mut Cpu, _mmu: &mut Mmu) {
    panic!("error");
}

pub fn op_fe(cpu: &mut Cpu, mmu: &mut Mmu) {
    let value = mmu.read_byte(cpu.pc + 1);
    cpu.apply_sub8_with_flags(cpu.a, value, false);

    cpu.pc += 2;
    cpu.cycles += 2;
}

pub fn op_ff(cpu: &mut Cpu, mmu: &mut Mmu) {
    cpu.push_word(mmu,cpu.pc);
    cpu.pc = 0x38;

    cpu.pc += 1;
    cpu.cycles += 4;
}