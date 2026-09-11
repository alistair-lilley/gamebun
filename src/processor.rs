// GameBun processing structs 
use std::{thread, time};

struct Registers {
    A: u8,
    B: u8,
    C: u8,
    D: u8,
    E: u8,
    F: u8,
    H: u8,
    L: u8,
}

impl Registers {
    // 16-bit registers
    fn get_AF(&self) -> u16 {
        (self.A as u16) << 8 
        | self.F as u16
    }

    fn get_BC(&self) -> u16 {
        (self.B as u16) << 8
        | self.C as u16
    }

    fn get_DE(&self) -> u16 {
        (self.D as u16) << 8
        | self.E as u16
    }

    fn get_HL(&self) -> u16 {
        (self.H as u16) << 8
        | self.L as u16
    }

    fn set_AF(&mut self, value: u16) {
        self.A = ((value & 0xFF00) >> 8) as u8;
        self.F = (value & 0xFF) as u8;
    }

    fn set_BC(&mut self, value: u16) {
        self.B = ((value & 0xFF00) >> 8) as u8;
        self.C = (value & 0xFF) as u8;
    }

    fn set_DE(&mut self, value: u16) {
        self.D = ((value & 0xFF00) >> 8) as u8;
        self.E = (value & 0xFF) as u8;
    }

    fn set_HL(&mut self, value: u16) {
        self.H = ((value & 0xFF00) >> 8) as u8;
        self.L = (value & 0xFF) as u8;
    }
}

struct FlagsRegister {
    zero: bool, // Z
    subtract: bool, // N
    half_carry: bool, // H
    carry: bool // C
}

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

impl std::convert::From<FlagsRegister> for u8  {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero       { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION |
        (if flag.subtract   { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION |
        (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION |
        (if flag.carry      { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry
        }
    }
}

struct CPU {
    registers: Registers,
    pc: u16,
    sp: u16,
    ir: u32,
    wram: RAM,
}

struct RAM {
    memory: [u8; 0xFFFF]
}

const T_CYCLE_PULSE = time::Duration::from_nanos(238);

impl CPU {
    // Clock T-cycle
    fn T_cycle(&self, count: u8) {
        for cycle in 0..count {
            let now = time::Instant::now();
            thread::sleep(T_CYCLE_PULSE);
            assert!(now.elapsed() >= T_CYCLE_PULSE);
        }
    }
    
    // Clock M-cycle which is 4 T-cycles
    fn M_cycle(&self, count: u8) {
        for cycle in 0..count {
            self.T_cycle(4);
        }
    }
}