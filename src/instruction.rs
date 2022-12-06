/// High level instruction
use modular_bitfield::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Instruction {
    Mathmatical {
        func: MathFunction,
        src: Target,
        dst: Target,
    },
    Load {
        dst: Register,
        src: MemoryMethod,
    },
    Store {
        src: Register,
        dst: MemoryMethod,
    },
    Branch(BranchKind),
    Nop,
    Hault,
}

#[derive(BitfieldSpecifier, Copy, Clone, PartialEq, Eq, Debug)]
#[bits = 3]
pub enum MathFunction {
    And = 0b000,
    Or = 0b001,
    Xor = 0b010,
    Add = 0b011,
    Sub = 0b100,
    Inc = 0b101,
    Dec = 0b110,
    Not = 0b111,
}

#[derive(BitfieldSpecifier, Copy, Clone, PartialEq, Eq, Debug)]
#[bits = 2]
pub enum Target {
    Indirect = 0b00,
    Acc = 0b01,
    Mar = 0b10,
    Memory = 0b11,
}

#[derive(BitfieldSpecifier, Copy, Clone, PartialEq, Eq, Debug)]
#[bits = 1]
pub enum Register {
    Acc = 0,
    Mar = 1,
}

#[derive(BitfieldSpecifier, Copy, Clone, PartialEq, Eq, Debug)]
#[bits = 2]
pub enum MemoryMethod {
    Address = 0b00,
    Constant = 0b01,
    Indirect = 0b10,
}

#[derive(BitfieldSpecifier, Copy, Clone, PartialEq, Eq, Debug)]
#[bits = 3]
pub enum BranchKind {
    Bra = 0b000,
    Brz = 0b001,
    Bne = 0b010,
    Blt = 0b011,
    Ble = 0b100,
    Bgt = 0b101,
    Bge = 0b110,
}
