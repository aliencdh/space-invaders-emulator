use crate::*;

pub type Operation = fn(&mut State8080) -> ();

pub const fn init_operations() -> [Operation; 256] {
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

    val[0x76] = hlt;

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
    val[0x90] = sub_b;
    val[0x91] = sub_c;
    val[0x92] = sub_d;
    val[0x93] = sub_e;
    val[0x94] = sub_e;
    val[0x95] = sub_l;
    val[0x96] = sub_m;
    val[0x97] = sub_d;
    val[0x98] = sbb_b;
    val[0x99] = sbb_c;
    val[0x9a] = sbb_d;
    val[0x9b] = sbb_e;
    val[0x9c] = sbb_h;
    val[0x9d] = sbb_l;
    val[0x9e] = sbb_m;
    val[0x9f] = sbb_a;
    val[0xa0] = ana_b;
    val[0xa1] = ana_c;
    val[0xa2] = ana_d;
    val[0xa3] = ana_e;
    val[0xa4] = ana_h;
    val[0xa5] = ana_l;
    val[0xa6] = ana_m;
    val[0xa7] = ana_a;
    val[0xa8] = xra_b;
    val[0xa9] = xra_c;
    val[0xaa] = xra_d;
    val[0xab] = xra_e;
    val[0xac] = xra_h;
    val[0xad] = xra_l;
    val[0xae] = xra_m;
    val[0xaf] = xra_a;
    val[0xb0] = ora_b;
    val[0xb1] = ora_c;
    val[0xb2] = ora_d;
    val[0xb3] = ora_e;
    val[0xb4] = ora_h;
    val[0xb5] = ora_l;
    val[0xb6] = ora_m;
    val[0xb7] = ora_a;
    val[0xb8] = cmp_b;
    val[0xb9] = cmp_c;
    val[0xba] = cmp_d;
    val[0xbb] = cmp_e;
    val[0xbc] = cmp_h;
    val[0xbd] = cmp_l;
    val[0xbe] = cmp_m;
    val[0xbf] = cmp_a;

    val[0xc0] = rnz;
    val[0xc1] = pop_b;
    val[0xc2] = jnz;

    val[0xc5] = push_b;
    val[0xc4] = cnz;

    val[0xc6] = adi;
    val[0xc7] = rst_0;
    val[0xc8] = rz;

    val[0xca] = jz;

    val[0xcc] = cz;
    val[0xcd] = call;

    val[0xce] = aci;
    val[0xcf] = rst_1;
    val[0xd0] = rnc;
    val[0xd1] = pop_d;
    val[0xd2] = jnc;
    val[0xd3] = out;
    val[0xd4] = cnc;
    val[0xd5] = push_d;
    val[0xd6] = sui;
    val[0xd7] = rst_2;
    val[0xd8] = rc;

    val[0xda] = jc;
    val[0xdb] = in_8080;
    val[0xdc] = cc;

    val[0xde] = sbi;
    val[0xdf] = rst_3;
    val[0xe0] = rpo;
    val[0xe1] = pop_h;
    val[0xe2] = jpo;
    val[0xe3] = xthl;
    val[0xe4] = cpo;
    val[0xe5] = push_h;

    val[0xe6] = ani;
    val[0xe7] = rst_4;
    val[0xe8] = rpe;
    val[0xe9] = pchl;

    val[0xea] = jpe;

    val[0xec] = cpe;

    val[0xee] = xri;
    val[0xef] = rst_5;
    val[0xf0] = rp;
    val[0xf1] = pop_psw;
    val[0xf2] = jp;
    val[0xf3] = di;
    val[0xf4] = cp;
    val[0xf5] = push_psw;
    val[0xf6] = ori;
    val[0xf7] = rst_6;
    val[0xf8] = rm;
    val[0xf9] = sphl;
    val[0xfa] = jm;
    val[0xfb] = ei;
    val[0xfc] = cm;

    val[0xfe] = cpi;
    val[0xff] = rst_7;

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
    let bc = ((state.c as u16) << 8) | (state.b as u16);
    let answer = if bc == 0xffff { 0 } else { bc + 1 };

    state.c = (answer >> 8) as u8;
    state.b = answer as u8;
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
    let hl = ((state.l as u32) << 8) | (state.h as u32);
    let bc = ((state.c as u32) << 8) | (state.b as u32);
    let answer = hl + bc;
    state.l = (answer >> 8) as u8;
    state.h = answer as u8;
    state.cc.cy = if answer > 0xffff { 1 } else { 0 };
}

/// Opcode 0x0b
/// Does not affect the flags.
fn dcx_b(state: &mut State8080) {
    let bc = ((state.c as u16) << 8) | (state.b as u16);
    let answer = if bc == 0 { 0xffff } else { bc - 1 };

    state.c = (answer >> 8) as u8;
    state.b = answer as u8;
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
    let de = ((state.e as u16) << 8) | (state.d as u16);
    let answer = if de == 0xffff { 0 } else { de + 1 };
    state.e = (answer >> 8) as u8;
    state.d = answer as u8;
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
    let hl = ((state.l as u32) << 8) | (state.h as u32);
    let de = ((state.e as u32) << 8) | (state.d as u32);
    let answer = hl + de;
    state.l = (answer >> 8) as u8;
    state.h = answer as u8;
    state.cc.cy = if answer > 0xffff { 1 } else { 0 };
}

/// Opcode 0x1b
fn dcx_d(state: &mut State8080) {
    let de = ((state.e as u16) << 8) | (state.d as u16);
    let answer = if de == 0 { 0xffff } else { de - 1 };
    state.e = (answer >> 8) as u8;
    state.d = answer as u8;
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
    let hl = ((state.l as u16) << 8) | (state.h as u16);
    let answer = if hl == 0xffff { 0 } else { hl + 1 };
    state.l = (answer >> 8) as u8;
    state.h = answer as u8;
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
    let hl = ((state.l as u32) << 8) | (state.h as u32);
    let answer = hl * 2;
    state.l = (answer >> 8) as u8;
    state.h = answer as u8;
    state.cc.cy = (answer > 0xffff) as u8;
}

/// Opcode 0x2b
fn dcx_h(state: &mut State8080) {
    let hl = ((state.l as u16) << 8) | (state.h as u16);
    let answer = if hl == 0 { 0xffff } else { hl - 1 };
    state.l = (answer >> 8) as u8;
    state.h = answer as u8;
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
    let hl = ((state.l as u32) << 8) | (state.h as u32);
    let answer = hl + state.sp as u32;
    state.l = (answer >> 8) as u8;
    state.h = answer as u8;
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

/// Opcode 0x90
fn sub_b(state: &mut State8080) {
    let answer = state.a as i16 - state.b as i16;
    let actual = clamp(answer, 0, 0xff) as u8;
    state.cc.z = (actual == 0) as u8;
    state.cc.s = ((actual & 0x80) != 0) as u8;
    state.cc.cy = (answer < 0) as u8;
    state.cc.p = parity(actual);
    state.a = actual;
}

/// Opcode 0x91
fn sub_c(state: &mut State8080) {
    let answer = state.a as i16 - state.c as i16;
    let actual = clamp(answer, 0, 0xff) as u8;
    state.cc.z = (actual == 0) as u8;
    state.cc.s = ((actual & 0x80) != 0) as u8;
    state.cc.cy = (answer < 0) as u8;
    state.cc.p = parity(actual);
    state.a = actual;
}

/// Opcode 0x92
fn sub_d(state: &mut State8080) {
    let answer = state.a as i16 - state.d as i16;
    let actual = clamp(answer, 0, 0xff) as u8;
    state.cc.z = (actual == 0) as u8;
    state.cc.s = ((actual & 0x80) != 0) as u8;
    state.cc.cy = (answer < 0) as u8;
    state.cc.p = parity(actual);
    state.a = actual;
}

/// Opcode 0x93
fn sub_e(state: &mut State8080) {
    let answer = state.a as i16 - state.e as i16;
    let actual = clamp(answer, 0, 0xff) as u8;
    state.cc.z = (actual == 0) as u8;
    state.cc.s = ((actual & 0x80) != 0) as u8;
    state.cc.cy = (answer < 0) as u8;
    state.cc.p = parity(actual);
    state.a = actual;
}

/// Opcode 0x94
fn sub_h(state: &mut State8080) {
    let answer = state.a as i16 - state.h as i16;
    let actual = clamp(answer, 0, 0xff) as u8;
    state.cc.z = (actual == 0) as u8;
    state.cc.s = ((actual & 0x80) != 0) as u8;
    state.cc.cy = (answer < 0) as u8;
    state.cc.p = parity(actual);
    state.a = actual;
}

/// Opcode 0x95
fn sub_l(state: &mut State8080) {
    let answer = state.a as i16 - state.l as i16;
    let actual = clamp(answer, 0, 0xff) as u8;
    state.cc.z = (actual == 0) as u8;
    state.cc.s = ((actual & 0x80) != 0) as u8;
    state.cc.cy = (answer < 0) as u8;
    state.cc.p = parity(actual);
    state.a = actual;
}

/// Opcode 0x96
fn sub_m(state: &mut State8080) {
    let offset = ((state.h as u16) << 8) | (state.l as u16);
    let answer = state.a as i16 - state.memory[offset as usize] as i16;
    let actual = clamp(answer, 0, 0xff) as u8;
    state.cc.z = (actual == 0) as u8;
    state.cc.s = ((actual & 0x80) != 0) as u8;
    state.cc.cy = (answer < 0) as u8;
    state.cc.p = parity(actual);
    state.a = actual;
}

/// Opcode 0x97
fn sub_a(state: &mut State8080) {
    let answer = state.a as i16 - state.a as i16;
    let actual = clamp(answer, 0, 0xff) as u8;
    state.cc.z = (actual == 0) as u8;
    state.cc.s = ((actual & 0x80) != 0) as u8;
    state.cc.cy = (answer < 0) as u8;
    state.cc.p = parity(actual);
    state.a = actual;
}

/// Opcode 0x98
fn sbb_b(state: &mut State8080) {
    let answer = state.a as i16 - state.b as i16 - state.cc.cy as i16;
    let actual = clamp(answer, 0, 0xff) as u8;
    state.cc.z = (actual == 0) as u8;
    state.cc.s = ((actual & 0x80) != 0) as u8;
    state.cc.cy = (answer < 0) as u8;
    state.cc.p = parity(actual);
    state.a = actual;
}

/// Opcode 0x99
fn sbb_c(state: &mut State8080) {
    let answer = state.a as i16 - state.c as i16 - state.cc.cy as i16;
    let actual = clamp(answer, 0, 0xff) as u8;
    state.cc.z = (actual == 0) as u8;
    state.cc.s = ((actual & 0x80) != 0) as u8;
    state.cc.cy = (answer < 0) as u8;
    state.cc.p = parity(actual);
    state.a = actual;
}

/// Opcode 0x9a
fn sbb_d(state: &mut State8080) {
    let answer = state.a as i16 - state.d as i16 - state.cc.cy as i16;
    let actual = clamp(answer, 0, 0xff) as u8;
    state.cc.z = (actual == 0) as u8;
    state.cc.s = ((actual & 0x80) != 0) as u8;
    state.cc.cy = (answer < 0) as u8;
    state.cc.p = parity(actual);
    state.a = actual;
}

/// Opcode 0x9b
fn sbb_e(state: &mut State8080) {
    let answer = state.a as i16 - state.e as i16 - state.cc.cy as i16;
    let actual = clamp(answer, 0, 0xff) as u8;
    state.cc.z = (actual == 0) as u8;
    state.cc.s = ((actual & 0x80) != 0) as u8;
    state.cc.cy = (answer < 0) as u8;
    state.cc.p = parity(actual);
    state.a = actual;
}

/// Opcode 0x9c
fn sbb_h(state: &mut State8080) {
    let answer = state.a as i16 - state.h as i16 - state.cc.cy as i16;
    let actual = clamp(answer, 0, 0xff) as u8;
    state.cc.z = (actual == 0) as u8;
    state.cc.s = ((actual & 0x80) != 0) as u8;
    state.cc.cy = (answer < 0) as u8;
    state.cc.p = parity(actual);
    state.a = actual;
}

/// Opcode 0x9d
fn sbb_l(state: &mut State8080) {
    let answer = state.a as i16 - state.l as i16 - state.cc.cy as i16;
    let actual = clamp(answer, 0, 0xff) as u8;
    state.cc.z = (actual == 0) as u8;
    state.cc.s = ((actual & 0x80) != 0) as u8;
    state.cc.cy = (answer < 0) as u8;
    state.cc.p = parity(actual);
    state.a = actual;
}

/// Opcode 0x9e
fn sbb_m(state: &mut State8080) {
    let offset = ((state.h as u16) << 8) | (state.l as u16);
    let answer = state.a as i16 - state.memory[offset as usize] as i16 - state.cc.cy as i16;
    let actual = clamp(answer, 0, 0xff) as u8;
    state.cc.z = (actual == 0) as u8;
    state.cc.s = ((actual & 0x80) != 0) as u8;
    state.cc.cy = (answer < 0) as u8;
    state.cc.p = parity(actual);
    state.a = actual;
}

/// Opcode 0x9f
fn sbb_a(state: &mut State8080) {
    let answer = state.a as i16 - state.a as i16 - state.cc.cy as i16;
    let actual = clamp(answer, 0, 0xff) as u8;
    state.cc.z = (actual == 0) as u8;
    state.cc.s = ((actual & 0x80) != 0) as u8;
    state.cc.cy = (answer < 0) as u8;
    state.cc.p = parity(actual);
    state.a = actual;
}

/// Opcode 0xa0
fn ana_b(state: &mut State8080) {
    state.a = state.a & state.b;
    state.cc.z = (state.a == 0) as u8;
    state.cc.s = ((state.a & 0x80) != 0) as u8;
    state.cc.cy = 0;
    state.cc.p = parity(state.a);
}

/// Opcode 0xa1
fn ana_c(state: &mut State8080) {
    state.a = state.a & state.c;
    state.cc.z = (state.a == 0) as u8;
    state.cc.s = ((state.a & 0x80) != 0) as u8;
    state.cc.cy = 0;
    state.cc.p = parity(state.a);
}
/// Opcode 0xa2
fn ana_d(state: &mut State8080) {
    state.a = state.a & state.d;
    state.cc.z = (state.a == 0) as u8;
    state.cc.s = ((state.a & 0x80) != 0) as u8;
    state.cc.cy = 0;
    state.cc.p = parity(state.a);
}

/// Opcode 0xa3
fn ana_e(state: &mut State8080) {
    state.a = state.a & state.e;
    state.cc.z = (state.a == 0) as u8;
    state.cc.s = ((state.a & 0x80) != 0) as u8;
    state.cc.cy = 0;
    state.cc.p = parity(state.a);
}

/// Opcode 0xa4
fn ana_h(state: &mut State8080) {
    state.a = state.a & state.h;
    state.cc.z = (state.a == 0) as u8;
    state.cc.s = ((state.a & 0x80) != 0) as u8;
    state.cc.cy = 0;
    state.cc.p = parity(state.a);
}

/// Opcode 0xa5
fn ana_l(state: &mut State8080) {
    state.a = state.a & state.l;
    state.cc.z = (state.a == 0) as u8;
    state.cc.s = ((state.a & 0x80) != 0) as u8;
    state.cc.cy = 0;
    state.cc.p = parity(state.a);
}

/// Opcode 0xa6
fn ana_m(state: &mut State8080) {
    let offset = ((state.h as u16) << 8) | (state.l as u16);
    state.a = state.a & state.memory[offset as usize];
    state.cc.z = (state.a == 0) as u8;
    state.cc.s = ((state.a & 0x80) != 0) as u8;
    state.cc.cy = 0;
    state.cc.p = parity(state.a);
}

/// Opcode 0xa7
fn ana_a(state: &mut State8080) {
    state.cc.z = (state.a == 0) as u8;
    state.cc.s = ((state.a & 0x80) != 0) as u8;
    state.cc.cy = 0;
    state.cc.p = parity(state.a);
}

/// Opcode 0xa8
fn xra_b(state: &mut State8080) {
    state.a = state.a ^ state.b;
    state.cc.z = (state.a == 0) as u8;
    state.cc.s = ((state.a & 0x80) != 0) as u8;
    state.cc.cy = 0;
    state.cc.p = parity(state.a);
}

/// Opcode 0xa9
fn xra_c(state: &mut State8080) {
    state.a = state.a ^ state.c;
    state.cc.z = (state.a == 0) as u8;
    state.cc.s = ((state.a & 0x80) != 0) as u8;
    state.cc.cy = 0;
    state.cc.p = parity(state.a);
}

/// Opcode 0xaa
fn xra_d(state: &mut State8080) {
    state.a = state.a ^ state.d;
    state.cc.z = (state.a == 0) as u8;
    state.cc.s = ((state.a & 0x80) != 0) as u8;
    state.cc.cy = 0;
    state.cc.p = parity(state.a);
}

/// Opcode 0xab
fn xra_e(state: &mut State8080) {
    state.a = state.a ^ state.e;
    state.cc.z = (state.a == 0) as u8;
    state.cc.s = ((state.a & 0x80) != 0) as u8;
    state.cc.cy = 0;
    state.cc.p = parity(state.a);
}

/// Opcode 0xac
fn xra_h(state: &mut State8080) {
    state.a = state.a ^ state.h;
    state.cc.z = (state.a == 0) as u8;
    state.cc.s = ((state.a & 0x80) != 0) as u8;
    state.cc.cy = 0;
    state.cc.p = parity(state.a);
}

/// Opcode 0xad
fn xra_l(state: &mut State8080) {
    state.a = state.a ^ state.l;
    state.cc.z = (state.a == 0) as u8;
    state.cc.s = ((state.a & 0x80) != 0) as u8;
    state.cc.cy = 0;
    state.cc.p = parity(state.a);
}

/// Opcode 0xae
fn xra_m(state: &mut State8080) {
    let offset = ((state.h as u16) << 8) | (state.l as u16);
    state.a = state.a ^ state.memory[offset as usize];
    state.cc.z = (state.a == 0) as u8;
    state.cc.s = ((state.a & 0x80) != 0) as u8;
    state.cc.cy = 0;
    state.cc.p = parity(state.a);
}

/// Opcode 0xaf
fn xra_a(state: &mut State8080) {
    state.a = state.a ^ state.a;
    state.cc.z = (state.a == 0) as u8;
    state.cc.s = ((state.a & 0x80) != 0) as u8;
    state.cc.cy = 0;
    state.cc.p = parity(state.a);
}

/// Opcoe 0xb0
fn ora_b(state: &mut State8080) {
    state.a = state.a | state.b;
    state.cc.z = (state.a == 0) as u8;
    state.cc.s = ((state.a & 0x80) != 0) as u8;
    state.cc.cy = 0;
    state.cc.p = parity(state.a);
}

/// Opcoe 0xb1
fn ora_c(state: &mut State8080) {
    state.a = state.a | state.c;
    state.cc.z = (state.a == 0) as u8;
    state.cc.s = ((state.a & 0x80) != 0) as u8;
    state.cc.cy = 0;
    state.cc.p = parity(state.a);
}

/// Opcoe 0xb2
fn ora_d(state: &mut State8080) {
    state.a = state.a | state.d;
    state.cc.z = (state.a == 0) as u8;
    state.cc.s = ((state.a & 0x80) != 0) as u8;
    state.cc.cy = 0;
    state.cc.p = parity(state.a);
}

/// Opcoe 0xb3
fn ora_e(state: &mut State8080) {
    state.a = state.a | state.e;
    state.cc.z = (state.a == 0) as u8;
    state.cc.s = ((state.a & 0x80) != 0) as u8;
    state.cc.cy = 0;
    state.cc.p = parity(state.a);
}

/// Opcoe 0xb4
fn ora_h(state: &mut State8080) {
    state.a = state.a | state.h;
    state.cc.z = (state.a == 0) as u8;
    state.cc.s = ((state.a & 0x80) != 0) as u8;
    state.cc.cy = 0;
    state.cc.p = parity(state.a);
}

/// Opcoe 0xb5
fn ora_l(state: &mut State8080) {
    state.a = state.a | state.l;
    state.cc.z = (state.a == 0) as u8;
    state.cc.s = ((state.a & 0x80) != 0) as u8;
    state.cc.cy = 0;
    state.cc.p = parity(state.a);
}

/// Opcoe 0xb6
fn ora_m(state: &mut State8080) {
    let offset = ((state.h as u16) << 8) | (state.l as u16);
    state.a = state.a | state.memory[offset as usize];
    state.cc.z = (state.a == 0) as u8;
    state.cc.s = ((state.a & 0x80) != 0) as u8;
    state.cc.cy = 0;
    state.cc.p = parity(state.a);
}

/// Opcoe 0xb7
fn ora_a(state: &mut State8080) {
    state.cc.z = (state.a == 0) as u8;
    state.cc.s = ((state.a & 0x80) != 0) as u8;
    state.cc.cy = 0;
    state.cc.p = parity(state.a);
}

/// Opcode 0xc2
fn jnz(state: &mut State8080) {
    state.pc = if 0 == state.cc.z {
        ((state.memory[(state.pc + 2) as usize] as u16) << 8)
            | (state.memory[(state.pc + 1) as usize] as u16)
    } else {
        state.pc + 2
    };
}

/// Opcode 0xc6
/// Immediate form: the addend is the byte after the instruction.
fn adi(state: &mut State8080) {
    let addend = state.memory[(state.pc + 1) as usize];
    let answer = state.a as u16 + addend as u16;
    state.cc.z = if (answer & 0xff) == 0 { 1 } else { 0 };
    state.cc.s = if (answer & 0x80) != 0 { 1 } else { 0 };
    state.cc.cy = if answer > 0xff { 1 } else { 0 };
    state.cc.p = parity(answer as u8);
    state.a = answer as u8;
    state.sp += 1;
}

/// Opcode 0xce
fn aci(state: &mut State8080) {
    let addend = state.memory[(state.pc + 1) as usize];
    let answer = state.a as u16 + addend as u16 + state.cc.cy as u16;
    state.cc.z = ((answer as u8) == 0) as u8;
    state.cc.s = ((answer & 0x80) != 0) as u8;
    state.cc.cy = (answer > 0xff) as u8;
    state.a = answer as u8;
    state.sp += 1;
}

/// Opcode 0xd6
fn sui(state: &mut State8080) {
    let addend = state.memory[(state.pc + 1) as usize];
    let answer = state.a as i16 - addend as i16;
    let actual = clamp(answer, 0, 0xff) as u8;
    state.cc.z = (actual == 0) as u8;
    state.cc.s = ((actual & 0x80) != 0) as u8;
    state.cc.cy = (answer < 0) as u8;
    state.a = actual;
    state.sp += 1;
}

/// Opcode 0xde
fn sbi(state: &mut State8080) {
    let addend = state.memory[(state.pc + 1) as usize];
    let answer = state.a as i16 - addend as i16 - state.cc.cy as i16;
    let actual = clamp(answer, 0, 0xff) as u8;
    state.cc.z = (actual == 0) as u8;
    state.cc.s = ((actual & 0x80) != 0) as u8;
    state.cc.cy = (answer < 0) as u8;
    state.a = actual;
    state.sp += 1;
}

/// Opcode 0xe6
fn ani(state: &mut State8080) {
    let data = state.memory[(state.pc + 1) as usize];
    let answer = state.a & data;
    state.cc.z = (answer == 0) as u8;
    state.cc.s = ((answer & 0x80) != 0) as u8;
    state.cc.cy = 0;
    state.a = answer;
    state.sp += 1;
}

/// Opcode 0xee
fn xri(state: &mut State8080) {
    let data = state.memory[(state.pc + 1) as usize];
    let answer = state.a | data;
    state.cc.z = (answer == 0) as u8;
    state.cc.s = ((answer & 0x80) != 0) as u8;
    state.cc.cy = 0;
    state.a = answer;
    state.sp += 1;
}

/// Opcode 0xf6
fn ori(state: &mut State8080) {
    let data = state.memory[(state.pc + 1) as usize];
    let answer = state.a | data;
    state.cc.z = (answer == 0) as u8;
    state.cc.s = ((answer & 0x80) != 0) as u8;
    state.cc.cy = 0;
    state.a = answer;
    state.sp += 1;
}

/// Opcode 0xca
fn jz(state: &mut State8080) {
    state.pc = if 0 != state.cc.z {
        ((state.memory[(state.pc + 2) as usize] as u16) << 8)
            | (state.memory[(state.pc + 1) as usize] as u16)
    } else {
        state.pc + 2
    };
}

/// Opcode 0xd2
fn jnc(state: &mut State8080) {
    state.pc = if state.cc.cy == 0 {
        ((state.memory[(state.pc + 2) as usize] as u16) << 8)
            | (state.memory[(state.pc + 1) as usize] as u16)
    } else {
        state.pc + 2
    }
}

/// Opcode 0xda
fn jc(state: &mut State8080) {
    state.pc = if state.cc.cy != 0 {
        ((state.memory[(state.pc + 2) as usize] as u16) << 8)
            | (state.memory[(state.pc + 1) as usize] as u16)
    } else {
        state.pc + 2
    }
}

/// Opcode 0xe2
fn jpo(state: &mut State8080) {
    state.pc = if state.cc.p == 0 {
        ((state.memory[(state.pc + 2) as usize] as u16) << 8)
            | (state.memory[(state.pc + 1) as usize] as u16)
    } else {
        state.pc + 2
    }
}

/// Opcode 0xea
fn jpe(state: &mut State8080) {
    state.pc = if state.cc.p != 0 {
        ((state.memory[(state.pc + 2) as usize] as u16) << 8)
            | (state.memory[(state.pc + 1) as usize] as u16)
    } else {
        state.pc + 2
    }
}

/// Opcode 0xf2
fn jp(state: &mut State8080) {
    state.pc = if state.cc.s == 0 {
        ((state.memory[(state.pc + 2) as usize] as u16) << 8)
            | (state.memory[(state.pc + 1) as usize] as u16)
    } else {
        state.pc + 2
    }
}

/// Opcode 0xfa
fn jm(state: &mut State8080) {
    state.pc = if state.cc.s != 0 {
        ((state.memory[(state.pc + 2) as usize] as u16) << 8)
            | (state.memory[(state.pc + 1) as usize] as u16)
    } else {
        state.pc + 2
    }
}

/// Opcode 0xcd
fn call(state: &mut State8080) {
    // push address onto the stack
    state.memory[(state.sp - 1) as usize] = (state.pc >> 8) as u8;
    state.memory[(state.sp - 2) as usize] = state.pc as u8;
    state.sp -= 2;

    // call subroutine
    state.pc = ((state.memory[(state.pc + 2) as usize] as u16) << 8) | (state.memory[(state.pc + 1) as usize] as u16);
}

/// Opcode 0xc9
fn ret(state: &mut State8080) {
    // set new program counter
    let lo = state.memory[state.sp as usize] as u16;
    let hi = state.memory[(state.sp + 1) as usize] as u16;
    state.pc = (hi << 8) | lo;

    // pop address off the stack
    state.sp += 2;
}

/// Opcode 0xcc
fn cz(state: &mut State8080) {
    if state.cc.z != 0 {
        call(state);
    } else {
        state.pc += 2;
    }
}

/// Opcode 0xc8
fn rz(state: &mut State8080) {
    if state.cc.z != 0 {
        ret(state);
    } else {
        state.pc += 2;
    }
}

/// Opcode 0xc4
fn cnz(state: &mut State8080) {
    if state.cc.z == 0 {
        call(state);
    } else {
        state.pc += 2;
    }
}

/// Opcode 0xc0
fn rnz(state: &mut State8080) {
    if state.cc.z == 0 {
        ret(state);
    } else {
        state.pc += 2;
    }
}

/// Opcode 0xdc
fn cc(state: &mut State8080) {
    if state.cc.cy != 0 {
        call(state);
    } else {
        state.pc += 2;
    }
}

/// Opcode 0xd8
fn rc(state: &mut State8080) {
    if state.cc.cy != 0 {
        ret(state);
    } else {
        state.pc += 2;
    }
}

/// Opcode 0xd4
fn cnc(state: &mut State8080) {
    if state.cc.cy == 0 {
        call(state);
    } else {
        state.pc += 2;
    }
}

/// Opcode 0xd0
fn rnc(state: &mut State8080) {
    if state.cc.cy == 0 {
        ret(state);
    } else {
        state.pc += 2;
    }
}

/// Opcode 0xe4
fn cpo(state: &mut State8080) {
    if state.cc.p == 0 {
        call(state);
    } else {
        state.pc += 2;
    }
}

/// Opcode 0xe0
fn rpo(state: &mut State8080) {
    if state.cc.p == 0 {
        ret(state);
    } else {
        state.pc += 2;
    }
}

/// Opcode 0xe8
fn cpe(state: &mut State8080) {
    if state.cc.p != 0 {
        call(state);
    } else {
        state.pc += 2;
    }
}

/// Opcode 0xec
fn rpe(state: &mut State8080) {
    if state.cc.p != 0 {
        ret(state);
    } else {
        state.pc += 2;
    }
}

/// Opcode 0xf4
fn cp(state: &mut State8080) {
    if state.cc.s == 0 {
        call(state);
    } else {
        state.pc += 2;
    }
}

/// Opcode 0xf0
fn rp(state: &mut State8080) {
    if state.cc.s == 0 {
        ret(state);
    } else {
        state.pc += 2;
    }
}

/// Opcode 0xf8
fn cm(state: &mut State8080) {
    if state.cc.s != 0 {
        call(state);
    } else {
        state.pc += 2;
    }
}

/// Opcode 0xfc
fn rm(state: &mut State8080) {
    if state.cc.s != 0 {
        ret(state);
    } else {
        state.pc += 2;
    }
}

/// Opcode 0xc7
fn rst_0(state: &mut State8080) {
    // push address onto the stack
    state.memory[(state.sp - 1) as usize] = (state.pc >> 8) as u8;
    state.memory[(state.sp - 2) as usize] = state.pc as u8;
    state.sp -= 2;

    // jump to $0
    state.pc = 0;
}

/// Opcode 0xcf
fn rst_1(state: &mut State8080) {
    // push address onto the stack
    state.memory[(state.sp - 1) as usize] = (state.pc >> 8) as u8;
    state.memory[(state.sp - 2) as usize] = state.pc as u8;
    state.sp -= 2;

    // jump to $8
    state.pc = 8;
}

/// Opcode 0xd7
fn rst_2(state: &mut State8080) {
    // push address onto the stack
    state.memory[(state.sp - 1) as usize] = (state.pc >> 8) as u8;
    state.memory[(state.sp - 2) as usize] = state.pc as u8;
    state.sp -= 2;

    // jump to $10
    state.pc = 10;
}

/// Opcode 0xdf
fn rst_3(state: &mut State8080) {
    // push address onto the stack
    state.memory[(state.sp - 1) as usize] = (state.pc >> 8) as u8;
    state.memory[(state.sp - 2) as usize] = state.pc as u8;
    state.sp -= 2;

    // jump to $18
    state.pc = 18;
}

/// Opcode 0xe7
fn rst_4(state: &mut State8080) {
    // push address onto the stack
    state.memory[(state.sp - 1) as usize] = (state.pc >> 8) as u8;
    state.memory[(state.sp - 2) as usize] = state.pc as u8;
    state.sp -= 2;

    // jump to $20
    state.pc = 20;
}

/// Opcode 0xef
fn rst_5(state: &mut State8080) {
    // push address onto the stack
    state.memory[(state.sp - 1) as usize] = (state.pc >> 8) as u8;
    state.memory[(state.sp - 2) as usize] = state.pc as u8;
    state.sp -= 2;

    // jump to $28
    state.pc = 28;
}

/// Opcode 0xf7
fn rst_6(state: &mut State8080) {
    // push address onto the stack
    state.memory[(state.sp - 1) as usize] = (state.pc >> 8) as u8;
    state.memory[(state.sp - 2) as usize] = state.pc as u8;
    state.sp -= 2;

    // jump to $30
    state.pc = 30;
}

/// Opcode 0xff
fn rst_7(state: &mut State8080) {
    // push address onto the stack
    state.memory[(state.sp - 1) as usize] = (state.pc >> 8) as u8;
    state.memory[(state.sp - 2) as usize] = state.pc as u8;
    state.sp -= 2;

    // jump to $38
    state.pc = 38;
}

/// Opcode 0xe9
fn pchl(state: &mut State8080) {
    let hi = state.h as u16;
    let lo = state.l as u16;
    state.pc = (hi << 8) | lo;
}

/// Opcode 0xb8
fn cmp_b(state: &mut State8080) {
    let a = state.a;
    sub_b(state);
    state.a = a;
}

/// Opcode 0xb9
fn cmp_c(state: &mut State8080) {
    let a = state.a;
    sub_c(state);
    state.a = a;
}

/// Opcode 0xba
fn cmp_d(state: &mut State8080) {
    let a = state.a;
    sub_d(state);
    state.a = a;
}


/// Opcode 0xbb
fn cmp_e(state: &mut State8080) {
    let a = state.a;
    sub_e(state);
    state.a = a;
}

/// Opcode 0xbc
fn cmp_h(state: &mut State8080) {
    let a = state.a;
    sub_h(state);
    state.a = a;
}

/// Opcode 0xbd
fn cmp_l(state: &mut State8080) {
    let a = state.a;
    sub_l(state);
    state.a = a;
}


/// Opcode 0xbe
fn cmp_m(state: &mut State8080) {
    let a = state.a;
    sub_m(state);
    state.a = a;
}


/// Opcode 0xbf
fn cmp_a(state: &mut State8080) {
    let a = state.a;
    sub_a(state);
    state.a = a;
}

/// Opcode 0xfe
fn cpi(state: &mut State8080) {
    let a = state.a;
    sbi(state);
    state.a = a;
}

/// Opcode 0xfb
fn ei(state: &mut State8080) {
    state.interrupt_enabled = true;
}

/// Opcode 0xf3
fn di(state: &mut State8080) {
    state.interrupt_enabled = false;
}

/// Opcode 0xdb
fn in_8080(state: &mut State8080) {
    state.pc += 1;
}

/// Opcode 0xd3
fn out(state: &mut State8080) {
    state.pc += 1;
}

/// Opcode 0xc1
fn pop_b(state: &mut State8080) {
    state.b = state.memory[(state.sp + 1 ) as usize];
    state.c = state.memory[state.sp as usize];
    state.sp += 2;
}

/// Opcode 0xc5
fn push_b(state: &mut State8080) {
    state.memory[(state.sp - 1) as usize] = state.b;
    state.memory[(state.sp - 2) as usize] = state.c;
    state.sp -= 2;
}

/// Opcode 0xd1
fn pop_d(state: &mut State8080) {
    state.d = state.memory[(state.sp + 1 ) as usize];
    state.e = state.memory[state.sp as usize];
    state.sp += 2;
}

/// Opcode 0xd5
fn push_d(state: &mut State8080) {
    state.memory[(state.sp - 1) as usize] = state.d;
    state.memory[(state.sp - 2) as usize] = state.e;
    state.sp -= 2;
}

/// Opcode 0xe1
fn pop_h(state: &mut State8080) {
    state.h = state.memory[(state.sp + 1 ) as usize];
    state.l = state.memory[state.sp as usize];
    state.sp += 2;
}

/// Opcode 0xe5
fn push_h(state: &mut State8080) {
    state.memory[(state.sp - 1) as usize] = state.h;
    state.memory[(state.sp - 2) as usize] = state.l;
    state.sp -= 2;
}

/// Opcode 0xf1
fn pop_psw(state: &mut State8080) {
    state.a = state.memory[(state.sp + 1) as usize];
    let psw = state.memory[state.sp as usize];
    state.cc.z = (0x01 == (psw & 0x01)) as u8;
    state.cc.s = (0x02 == (psw & 0x02)) as u8;
    state.cc.p = (0x04 == (psw & 0x03)) as u8;
    state.cc.cy = (0x05 == (psw & 0x05)) as u8;
    state.cc.ac = (0x10 == (psw & 0x10)) as u8;
    state.sp += 1;
}

/// Opcode 0xf5
fn push_psw(state: &mut State8080) {
    state.memory[(state.sp - 1) as usize] = state.a;
    let psw = state.cc.z |
        (state.cc.s << 1) |
        (state.cc.p << 2) |
        (state.cc.cy << 3) |
        (state.cc.ac << 4);
    state.memory[(state.sp - 2) as usize] = psw;
    state.sp -= 2;
}

/// Opcode 0xf9
fn sphl(state: &mut State8080) {
    state.sp = ((state.l as u16) << 8) | (state.h as u16);
}

/// Opcode 0xe3
fn xthl(state: &mut State8080) {
    state.h = state.sp as u8;
    state.l = (state.sp >> 8) as u8;
}

/// This function does nothing and is used as the NOP operation because Rust won't accept a closure.
fn nop(_: &mut State8080) {}

/// Opcode 0x76
/// Exits the program.
fn hlt(_: &mut State8080) {
    panic!("HLT opcode encountered.");
}
