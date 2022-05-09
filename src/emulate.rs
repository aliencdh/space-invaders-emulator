use crate::*;

pub type Operation = fn(&mut State8080) -> ();

const fn init_operations() -> [Operation; 256] {
    let mut val: [Operation; 256] = [unimplemented_instruction; 256];

    val[0x00] = nop;
    val[0x01] = lxi_b;

    // arithmetic group
    val[0x03] = inx_b;
    val[0x04] = inr_b;
    val[0x05] = dcr_b;

    val[0x09] = dad_b;

    val[0x0b] = dcx_b;
    val[0x0c] = inr_c;
    val[0x0d] = dcr_c;

    val[0x0f] = rrc;

    val[0x13] = inx_d;
    val[0x14] = inr_d;
    val[0x15] = dcr_d;

    val[0x17] = ral;
    val[0x19] = dad_d;

    val[0x1b] = dcx_d;
    val[0x1c] = inr_e;
    val[0x1d] = dcr_e;

    val[0x1f] = rar;

    val[0x23] = inx_h;
    val[0x24] = inr_h;
    val[0x25] = dcr_h;

    val[0x29] = dad_h;

    val[0x2b] = dcx_h;
    val[0x2c] = inr_l;
    val[0x2d] = dcr_l;

    val[0x2f] = cma;

    val[0x33] = inx_sp;
    val[0x34] = inr_m;
    val[0x35] = dcr_m;

    val[0x37] = stc;
    val[0x39] = dad_sp;
    val[0x3b] = dcx_sp;
    val[0x3c] = inr_a;
    val[0x3d] = dcr_a;

    val[0x3f] = cmc;

    val[0x80] = add_b;
    val[0x81] = add_c;
    val[0x82] = add_d;
    val[0x83] = add_e;
    val[0x84] = add_h;
    val[0x85] = add_l;
    val[0x86] = add_m;
    val[0x87] = add_a;
    val[0x88] = adc_b;
    val[0x89] = adc_c;
    val[0x8a] = adc_d;
    val[0x8b] = adc_e;
    val[0x8c] = adc_h;
    val[0x8d] = adc_l;
    val[0x8e] = adc_m;
    val[0x8f] = adc_a;

    val[0xc6] = adi;

    val[0x41] = mov_b_c;
    val[0x42] = mov_b_d;
    val[0x43] = mov_b_e;

    val
}

pub const OPERATIONS: [Operation; 256] = init_operations();
   
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

/// Opcode 0x03
/// Does not affect the flags.
fn inx_b(state: &mut State8080) {
    let bc = ((state.b as u16) << 8) | (state.c as u16);
    let answer = if bc == 0xffff { 0 } else { bc + 1 };

    state.b = (answer >> 8) as u8;
    state.c = answer as u8;
}

/// Opcode 0x04
/// Does not affect the carry flag.
fn inr_b(state: &mut State8080) {
    state.b = if state.b == 0xff { 0 } else { state.b + 1 };
    state.cc.z = if state.b == 0 { 1 } else { 0 };
    state.cc.s = if (state.b & 0x80) != 0 { 1 } else { 0 };
    state.cc.p = parity(state.b);
}

/// Opcode 0x05
/// Does not affect the carry flag.
fn dcr_b(state: &mut State8080) {
    state.b = if state.b == 0 { 0xff } else { state.b - 1 };
    state.cc.z = if state.b == 0 { 1 } else { 0 }; 
    state.cc.s = if (state.b & 0x80) != 0 { 1 } else { 0 };
    state.cc.p = parity(state.b);
}

/// Opcode 0x09
/// Only affects the carry flag.
fn dad_b(state: &mut State8080) {
    let hl = ((state.h as u32) << 8) | (state.l as u32);
    let bc = ((state.b as u32) << 8) | (state.c as u32);
    let answer = hl + bc;
    state.cc.cy = if answer > 0xffff { 1 } else { 0 };
}

/// Opcode 0x0b
/// Does not affect the flags.
fn dcx_b(state: &mut State8080) {
    let bc = ((state.b as u16) << 8) | (state.c as u16);
    let answer = if bc == 0 { 0xffff } else { bc - 1 };

    state.b = (answer >> 8) as u8;
    state.c = answer as u8;
}

/// Opcode 0x0c
fn inr_c(state: &mut State8080) {
    state.c = if state.c == 0xff { 0 } else { state.c + 1 };
    state.cc.z = if state.c == 0 { 1 } else { 0 };
    state.cc.s = if (state.c & 0x80) != 0 { 1 } else { 0 };
    state.cc.p = parity(state.c);
}

/// Opcode 0x0d
fn dcr_c(state: &mut State8080) {
    state.c = if state.c == 0 { 0xff } else { state.c - 1 };
    state.cc.z = if state.c == 0 { 1 } else { 0 };
    state.cc.s = if (state.c & 0x80) != 0 { 1 } else { 0 };
    state.cc.p = parity(state.c);
}

/// Opcode 0x0f
fn rrc(state: &mut State8080) {
    state.cc.cy = state.a & 1;
    state.a >>= 1;
}

/// Opcode 0x13
fn inx_d(state: &mut State8080) {
    let de = ((state.d as u16) << 8) | (state.e as u16);
    let answer = if de == 0xffff { 0 } else { de + 1 };
    state.d = (answer >> 8) as u8;
    state.e = answer as u8;
}

/// Opcode 0x14
fn inr_d(state: &mut State8080) {
    state.d = if state.d == 0xff { 0 } else { state.d + 1 };
    state.cc.z = if state.d == 0 { 1 } else { 0 };
    state.cc.s = if (state.c & 0x80) != 0 { 1 } else { 0 };
    state.cc.p = parity(state.c);
}

/// Opcode 0x15
fn dcr_d(state: &mut State8080) {
    state.d = if state.d == 0 { 0xff } else { state.d - 1 };
    state.cc.z = if state.d == 0 { 1 } else { 0 };
    state.cc.s = if (state.c & 0x80) != 0 { 1 } else { 0 };
    state.cc.p = parity(state.c);
}

/// Opcode 0x17
fn ral(state: &mut State8080) {
    state.cc.cy = state.a & (1 << 7);
    state.a <<= 1;
}

/// Opcode 0x19
fn dad_d(state: &mut State8080) {
    let hl = ((state.h as u32) << 8) | (state.l as u32);
    let de = ((state.d as u32) << 8) | (state.e as u32);
    let answer = hl + de;
    state.h = (answer >> 8) as u8;
    state.l = answer as u8;
    state.cc.cy = if answer > 0xffff { 1 } else { 0 };
}

/// Opcode 0x1b
fn dcx_d(state: &mut State8080) {
    let de = ((state.d as u16) << 8) | (state.e as u16);
    let answer = if de == 0 { 0xffff } else { de - 1 };
    state.d = (answer >> 8) as u8;
    state.e = answer as u8;
}

/// Opcode 0x1c
fn inr_e(state: &mut State8080) {
    state.e = if state.e == 0xff { 0 } else { state.e + 1 };
    state.cc.z = if state.e == 0 { 1 } else { 0 };
    state.cc.s = if (state.e & 0x80) != 0 { 1 } else { 0 };
    state.cc.p = parity(state.e);
}

/// Opcode 0x1d
fn dcr_e(state: &mut State8080) {
    state.e = if state.e == 0 { 0xff } else { state.e - 1 };
    state.cc.z = if state.e == 0 { 1 } else { 0 };
    state.cc.s = if (state.e & 0x80) != 0 { 1 } else { 0 };
    state.cc.p = parity(state.e);
}

/// Opcode 0x1f
fn rar(state: &mut State8080) {
    state.cc.cy = state.a & 1;
    state.a = state.a >> 1;
}

/// Opcode 0x23
fn inx_h(state: &mut State8080) {
    let hl = ((state.h as u16) << 8) | (state.l as u16);
    let answer = if hl == 0xffff { 0 } else { hl + 1 };
    state.h = (answer >> 8) as u8;
    state.l = answer as u8;
}

/// Opcode 0x24
fn inr_h(state: &mut State8080) {
    state.h = if state.h == 0xff { 0 } else { state.h + 1 };
    state.cc.z = if state.h == 0 { 1 } else { 0 };
    state.cc.s = if (state.h & 0x80) != 0 { 1 } else { 0 };
    state.cc.p = parity(state.h);
}

/// Opcode 0x25
fn dcr_h(state: &mut State8080) {
    state.h = if state.h == 0 { 0xff } else { state.h - 1 };
    state.cc.z = if state.h == 0 { 1 } else { 0 };
    state.cc.s = if (state.h & 0x80) != 0 { 1 } else { 0 };
    state.cc.p = parity(state.h);
}

/// Opcode 0x29
fn dad_h(state: &mut State8080) {
    let hl = ((state.h as u32) << 8) | (state.l as u32);
    let answer = hl * 2;
    state.h = (answer >> 8) as u8;
    state.l = answer as u8;
    state.cc.cy = (answer > 0xffff) as u8;
}

/// Opcode 0x2b
fn dcx_h(state: &mut State8080) {
    let hl = ((state.h as u16) << 8) | (state.l as u16);
    let answer = if hl == 0 { 0xffff } else { hl - 1 };
    state.h = (answer >> 8) as u8;
    state.l = answer as u8;
}

/// Opcode 0x2c
fn inr_l(state: &mut State8080) {
    state.l = if state.l == 0xff { 0 } else { state.l + 1 };
    state.cc.z = (state.l == 0) as u8;
    state.cc.s = ((state.l & 0x80) != 0) as u8;
    state.cc.p = parity(state.l);
}

/// Opcode 0x2d
fn dcr_l(state: &mut State8080) {
    state.l = if state.l == 0 { 0xff } else { state.l - 1 };
    state.cc.z = (state.l == 0) as u8;
    state.cc.s = ((state.l & 0x80) != 0) as u8;
    state.cc.p = parity(state.l);
}

/// Opcode 0x2f
fn cma(state: &mut State8080) {
    state.a = !state.a;
}

/// Opcode 0x33
fn inx_sp(state: &mut State8080) {
    state.sp += 1;
}

/// Opcode 0x34
fn inr_m(state: &mut State8080) {
    let offset = ((state.h as u16) << 8) | (state.l as u16);
    let m = state.memory[offset as usize];
    let answer = if m == 0xff { 0 } else { m + 1 };
    state.memory[offset as usize] = answer;

    state.cc.z = (answer == 0) as u8;
    state.cc.s = ((answer & 0x80) != 0) as u8;
    state.cc.p = parity(answer);
}

/// Opcode 0x35
fn dcr_m(state: &mut State8080) {
    let offset = ((state.h as u16) << 8) | (state.l as u16);
    let m = state.memory[offset as usize];
    let answer = if m == 0 { 0xff } else { m - 1 };
    state.memory[offset as usize] = answer;

    state.cc.z = (answer == 0) as u8;
    state.cc.s = ((answer & 0x80) != 0) as u8;
    state.cc.p = parity(answer);
}

/// Opcode 0x37
fn stc(state: &mut State8080) {
    state.cc.cy = 1;
}

/// Opcode 0x39
fn dad_sp(state: &mut State8080) {
    let hl = ((state.h as u32) << 8) | (state.l as u32);
    let answer = hl + state.sp as u32;
    state.h = (answer >> 8) as u8;
    state.l = answer as u8;
    state.cc.cy = (answer > 0xffff) as u8;
}

/// Opcode 0x3b
fn dcx_sp(state: &mut State8080) {
    state.sp -= 1;
}

/// Opcode 0x3c
fn inr_a(state: &mut State8080) {
    state.a = if state.a == 0xff { 0 } else { state.a + 1 };
    state.cc.z = (state.a == 0) as u8;
    state.cc.s = ((state.a & 0x80) != 0) as u8;
    state.cc.p = parity(state.a);
}

/// Opcode 0x3d
fn dcr_a(state: &mut State8080) {
    state.a = if state.a == 0 { 0xff } else { state.a - 1 };
    state.cc.z = (state.a == 0) as u8;
    state.cc.s = ((state.a & 0x80) != 0) as u8;
    state.cc.p = parity(state.a);
}

/// Opcode 0x3f
fn cmc(state: &mut State8080) {
    state.cc.cy = !state.cc.cy & 1;
}

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

/// Opcode 0x82
fn add_d(state: &mut State8080) {
    let answer = state.a as u16 + state.d as u16;
    state.cc.z = (answer == 0) as u8;
    state.cc.s = ((answer & 0x80) != 0) as u8;
    state.cc.cy = (answer > 0xff) as u8;
    state.cc.p = parity(answer as u8);
    state.a = answer as u8;
}

/// Opcode 0x83
fn add_e(state: &mut State8080) {
    let answer = state.a as u16 + state.e as u16;
    state.cc.z = (answer == 0) as u8;
    state.cc.s = ((answer & 0x80) != 0) as u8;
    state.cc.cy = (answer > 0xff) as u8;
    state.cc.p = parity(answer as u8);
    state.a = answer as u8;
}

/// Opcode 0x84
fn add_h(state: &mut State8080) {
    let answer = state.a as u16 + state.h as u16;
    state.cc.z = (answer == 0) as u8;
    state.cc.s = ((answer & 0x80) != 0) as u8;
    state.cc.cy = (answer > 0xff) as u8;
    state.cc.p = parity(answer as u8);
    state.a = answer as u8;
}

/// Opcode 0x85
fn add_l(state: &mut State8080) {
    let answer = state.a as u16 + state.l as u16;
    state.cc.z = (answer == 0) as u8;
    state.cc.s = ((answer & 0x80) != 0) as u8;
    state.cc.cy = (answer > 0xff) as u8;
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

/// Opcode 0x87
fn add_a(state: &mut State8080) {
    let answer = state.a as u16 * 2;
    state.cc.z = (answer == 0) as u8;
    state.cc.s = ((answer & 0x80) != 0) as u8;
    state.cc.cy = (answer > 0xff) as u8;
    state.cc.p = parity(answer as u8);
    state.a = answer as u8;
}

/// Opcode 0x88
fn adc_b(state: &mut State8080) {
    let answer = state.a as u16 + state.b as u16 + state.cc.cy as u16;
    state.cc.z = (answer == 0) as u8;
    state.cc.s = ((answer & 0x80) != 0) as u8;
    state.cc.cy = (answer > 0xff) as u8;
    state.cc.p = parity(answer as u8);
    state.a = answer as u8;
}

/// Opcode 0x89
fn adc_c(state: &mut State8080) {
    let answer = state.a as u16 + state.c as u16 + state.cc.cy as u16;
    state.cc.z = (answer == 0) as u8;
    state.cc.s = ((answer & 0x80) != 0) as u8;
    state.cc.cy = (answer > 0xff) as u8;
    state.cc.p = parity(answer as u8);
    state.a = answer as u8;
}

/// Opcode 0x8a
fn adc_d(state: &mut State8080) {
    let answer = state.a as u16 + state.d as u16 + state.cc.cy as u16;
    state.cc.z = (answer == 0) as u8;
    state.cc.s = ((answer & 0x80) != 0) as u8;
    state.cc.cy = (answer > 0xff) as u8;
    state.cc.p = parity(answer as u8);
    state.a = answer as u8;
}

/// Opcode 0x8b
fn adc_e(state: &mut State8080) {
    let answer = state.a as u16 + state.e as u16 + state.cc.cy as u16;
    state.cc.z = (answer == 0) as u8;
    state.cc.s = ((answer & 0x80) != 0) as u8;
    state.cc.cy = (answer > 0xff) as u8;
    state.cc.p = parity(answer as u8);
    state.a = answer as u8;
}

/// Opcode 0x8c
fn adc_h(state: &mut State8080) {
    let answer = state.a as u16 + state.h as u16 + state.cc.cy as u16;
    state.cc.z = (answer == 0) as u8;
    state.cc.s = ((answer & 0x80) != 0) as u8;
    state.cc.cy = (answer > 0xff) as u8;
    state.cc.p = parity(answer as u8);
    state.a = answer as u8;
}

/// Opcode 0x8d
fn adc_l(state: &mut State8080) {
    let answer = state.a as u16 + state.l as u16 + state.cc.cy as u16;
    state.cc.z = (answer == 0) as u8;
    state.cc.s = ((answer & 0x80) != 0) as u8;
    state.cc.cy = (answer > 0xff) as u8;
    state.cc.p = parity(answer as u8);
    state.a = answer as u8;
}

/// Opcode 0x8e
fn adc_m(state: &mut State8080) {
    let offset = ((state.h as u16) << 8) | (state.l as u16); // 8080 uses 16-bit addresses
    let answer = state.a as u16 + state.memory[offset as usize] as u16 + state.cc.cy as u16;

    state.cc.z = if (answer & 0xff) == 0 { 1 } else { 0 };
    state.cc.s = if (answer & 0x80) != 0 { 1 } else { 0 };
    state.cc.cy = if answer > 0xff { 1 } else { 0 };
    state.cc.p = parity(answer as u8);
    state.a = answer as u8;
}

/// Opcode 0x8f
fn adc_a(state: &mut State8080) {
    let answer = state.a as u16 * 2 + state.cc.cy as u16;
    state.cc.z = (answer == 0) as u8;
    state.cc.s = ((answer & 0x80) != 0) as u8;
    state.cc.cy = (answer > 0xff) as u8;
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
    state.sp += 1;
}

/// This function does nothing and is used as the NOP operation because Rust won't accept a closure.
fn nop(_: &mut State8080) {}
