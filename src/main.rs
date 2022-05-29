#![feature(const_mut_refs)]
use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use std::io::Read;

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
    pub sp: u16,
    pub pc: u16,
    pub memory: Vec<u8>,
    pub cc: ConditionCodes,
    pub int_enable: bool,
    pub interrupt_enabled: bool
}
impl State8080 {
    pub fn new(buf: &[u8]) -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
            memory: Vec::from(buf),
            cc: ConditionCodes::default(),
            int_enable: false,
            interrupt_enabled: false
        }
    }
}

pub fn unimplemented_instruction(state: &mut State8080) {
    panic!("Error: Unimplemented instruction at byte {}", state.pc);
} 

/// Returns 1 if the number is even, 0 if it's odd.
pub fn parity(x: u8) -> u8 {
    (x % 2 == 0) as u8
}

pub fn clamp<T: Ord>(given: T, min: T, max: T) -> T {
    if given < min {
        min
    } else if given > max {
        max
    } else {
        given
    }
}

fn main() -> Result<(), String> {
    let filename = String::from("space-invaders");
    let file = File::open(&Path::new(&filename)).map_err(|err| format!("{:?}", err))?;
    let mut reader = BufReader::new(file);

    let mut buf = vec![];
    reader.read_to_end(&mut buf).map_err(|err| format!("{:?}", err))?;

    let mut state = State8080::new(reader.buffer());

    loop {
        emulate_8080_op(&mut state);
    }
}

