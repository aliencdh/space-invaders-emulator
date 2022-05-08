#[macro_use] extern crate lazy_static;

mod emulate;
use emulate::*;

pub struct ConditionCodes {
    /// The "zero" flag.
    /// Set to 1 when the result is 0.
    pub z: u8,
    /// The "sign" flag.
    /// Set to 1 when bit 7 (the most significant bit or MSB) of the math instruction is set.
    pub s: u8,
    /// The "parity" flag.
    /// Set to 1 when the result is even, and to 0 when it's odd.
    pub p: u8,
    /// The "carry" flag.
    /// Set to 1 when the instruction resulted in a carry/borrow.
    pub cy: u8,
    /// The "auxiliary carry" flag.
    /// Used mostly for BCD(binary coded decimal) math.
    pub ac: u8,
    pub pad: u8,
}
impl Default for ConditionCodes {
    fn default() -> Self {
        Self {
            z: 1,
            s: 1,
            p: 1,
            cy: 1,
            ac: 1,
            pad: 3
        }
    }
}

/// The global state of an 8080 program.
pub struct State8080 {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u8,
    pub pc: u8,
    pub memory: Vec<u8>,
    pub cc: ConditionCodes,
    pub int_enable: u8,
}

pub fn unimplemented_instruction(_state: &mut State8080) {
    panic!("Error: Unimplemented instruction.");
} 

pub fn parity(_: u8) -> u8 { todo!() }

fn main() {
    println!("Hello, world!");
}

