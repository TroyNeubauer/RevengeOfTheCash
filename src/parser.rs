#![allow(non_snake_case)]

use crate::*;
use modular_bitfield::prelude::*;

#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct MathBits {
    _set_math_bit: B1,
    func: MathFunction,
    dst: Target,
    src: Target,
}

#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct MemoryBits {
    _set_memory_bits: B4,
    is_load: B1,
    register: Register,
    method: MemoryMethod,
}

#[bitfield]
#[derive(Debug, Copy, Clone)]
pub struct BranchBits {
    _set_branch_bits: B5,
    kind: BranchKind,
}

pub fn try_parse(opcode: u8) -> Option<Instruction> {
    Some(match opcode {
        0b1000_000..=0b1111_1111 => {
            // High bit set, followed by any payload bits: math
            let ins = MathBits::from_bytes([opcode]);
            dbg!(ins);
            Instruction::Mathmatical {
                func: ins.func(),
                src: ins.src(),
                dst: ins.dst(),
            }
        }
        0b0000_0000..=0b0000_1111 => {
            println!("{:0b}", opcode);
            // High bit set, followed by any payload bits: math
            let ins = MemoryBits::from_bytes([opcode]);
            dbg!(ins);
            if ins.is_load() == 1 {
                Instruction::Load {
                    dst: ins.register(),
                    src: ins.method(),
                }
            } else {
                Instruction::Store {
                    src: ins.register(),
                    dst: ins.method(),
                }
            }
        }
        0b0001_0000..=0b0001_0111 => {
            let ins = BranchBits::from_bytes([opcode]);
            dbg!(ins);
            Instruction::Branch(ins.kind())
        }
        0b0001_1000 => Instruction::Nop,
        0b0001_1100 => Instruction::Hault,
        _ => {
            // illegal Instruction
            return None;
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        assert_eq!(
            try_parse(0).unwrap(),
            Instruction::Load {
                dst: Register::Acc,
                src: MemoryMethod::Address
            }
        );
    }
}
