use crate::*;

lazy_static! {
    pub static ref OPERATIONS: Vec<fn(&mut State8080) -> ()> = {
        let mut val: Vec<fn(&mut State8080) -> ()> = Vec::with_capacity(256);

        for i in 0..val.len() {
            match i {
                0x00 => val[i] = nop,
                0x01 => val[i] = lxi_b,

                0x41 => val[i] = mov_b_c,
                0x42 => val[i] = mov_b_d,
                0x43 => val[i] = mov_b_e,

                // arithmetic group
                0x80 => val[i] = add_b,
                0x81 => val[i] = add_c,
                0x86 => val[i] = add_m,
                0xc6 => val[i] = adi,

                _ => val[i] = unimplemented_instruction
            }
        }
        
        val
    };
}

/// Executes the opcode at the current program counter and increments the program counter.
pub fn emulate_8080_op(state: &mut State8080) {
    let opcode = state.memory[(state.pc + 1) as usize];

    OPERATIONS[opcode as usize](state);

    state.pc += 1;
}

/// Opcode 0x01.
fn lxi_b(state: &mut State8080) {
    state.c = state.memory[(state.pc + 2) as usize]; // store 1st param in C
    state.b = state.memory[(state.pc + 3) as usize]; // store 2nd param in B

    state.pc += 2;
}

/// Opcode 0x41
fn mov_b_c(state: &mut State8080) {
    state.b = state.c;
}

/// Opcode 0x42
fn mov_b_d(state: &mut State8080) {
    state.b = state.d;
}

/// Opcode 0x43
fn mov_b_e(state: &mut State8080) {
    state.b = state.e;
}


//////////////////////////
//// Arithmetic Group ////
//////////////////////////
/// Opcode 0x80
fn add_b(state: &mut State8080) {
    // the math is done with u16 for higher precision, in order to capture any carries 
    let answer = state.a as u16 + state.b as u16;
    state.cc.z = if (answer & 0xff) == 0 { 1 } else { 0 };
    state.cc.s = if (answer & 0x80) != 0 { 1 } else { 0 };
    state.cc.cy = if answer > 0xff { 1 } else { 0 };
    state.cc.p = parity(answer as u8);
    state.a = answer as u8;
}

/// Opcode 0x81
fn add_c(state: &mut State8080) {
    let answer = state.a as u16 + state.c as u16;
    state.cc.z = if (answer & 0xff) == 0 { 1 } else { 0 };
    state.cc.s = if (answer & 0x80) != 0 { 1 } else { 0 };
    state.cc.cy = if answer > 0xff { 1 } else { 0 };
    state.cc.p = parity(answer as u8);
    state.a = answer as u8;
}

/// Opcode 0x86
/// Memory form: the addend is the byte pointed to by the address stored in the HL register pair.
fn add_m(state: &mut State8080) {
    let offset = ((state.h as u16) << 8) | (state.l as u16); // 8080 uses 16-bit addresses
    let answer = state.a as u16 + state.memory[offset as usize] as u16;

    state.cc.z = if (answer & 0xff) == 0 { 1 } else { 0 };
    state.cc.s = if (answer & 0x80) != 0 { 1 } else { 0 };
    state.cc.cy = if answer > 0xff { 1 } else { 0 };
    state.cc.p = parity(answer as u8);
    state.a = answer as u8;
}

/// Opcode 0xc6
/// Immediate form: the addend is the byte after the instruction.
fn adi(state: &mut State8080) {
    let addend = state.memory[(state.pc + 2) as usize];
    let answer = state.a as u16 + addend as u16;
    state.cc.z = if (answer & 0xff) == 0 { 1 } else { 0 };
    state.cc.s = if (answer & 0x80) != 0 { 1 } else { 0 };
    state.cc.cy = if answer > 0xff { 1 } else { 0 };
    state.cc.p = parity(answer as u8);
    state.a = answer as u8;
}

/// This function does nothing and is used as the NOP operation because Rust won't accept a closure.
fn nop(_: &mut State8080) {}
