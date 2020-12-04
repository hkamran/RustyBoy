use crate::cpu::Cpu;

pub type OperationType = fn(cpu: &mut Cpu) -> ();

#[allow(unreachable_patterns)]
pub fn get(opcode: u8) -> OperationType {
    match opcode {
        0x00 => { op_00 },
        _ => { panic!("opcode not found {}", opcode)}
    }
}

pub fn op_00(cpu: &mut Cpu) {
    println!("NOP");
    cpu.bus.borrow_mut().mutate();

    cpu.pc += 1;
    cpu.cycles += 2;
}