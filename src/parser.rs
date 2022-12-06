use crate::*;
use modular_bitfield::prelude::*;

pub fn try_parse(opcode: u8) -> Option<Instruction> {
    Some(match opcode {
        0b1000_000..=0b1111_1111 => {
            let func = MathFunction::from_bytes((opcode & 0b0111_0000) >> 4).ok()?;
            let dst = Target::from_bytes((opcode & 0b0000_1100) >> 2).ok()?;
            let src = Target::from_bytes((opcode & 0b0000_0011) >> 0).ok()?;
            Instruction::Mathmatical { func, src, dst }
        }
        0b0000_0000..=0b0000_1111 => {
            let reg = Register::from_bytes((opcode & 0b0100) >> 2).ok()?;
            let method = MemoryMethod::from_bytes(opcode & 0b0011).ok()?;
            if (opcode & 0b1000) >> 3 == 1 {
                Instruction::Load {
                    dst: reg,
                    src: method,
                }
            } else {
                Instruction::Store {
                    src: reg,
                    dst: method,
                }
            }
        }
        0b0001_0000..=0b0001_0111 => {
            let kind = BranchKind::from_bytes(opcode & 0b111).ok()?;
            Instruction::Branch(kind)
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
