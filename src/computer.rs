use crate::*;
use crate::{try_parse, DstTarget, Instruction, MathFunction};

pub const MEMORY_SIZE: usize = 64 * 1024;

pub struct Computer {
    memory: [u8; MEMORY_SIZE],
    acc: u8,
    ir: u8,
    mar: u16,
    pc: u16,
}

#[derive(PartialEq, Eq)]
pub enum ExecuteResult {
    Continue,
    Hault,
}

impl Computer {
    pub fn new(memory: [u8; MEMORY_SIZE]) -> Self {
        Self {
            memory,
            acc: 0,
            ir: 0,
            mar: 0,
            pc: 0,
        }
    }

    /// Starts executing at memory address 0, and runs until a hault instruction is encountered
    pub fn run(&mut self) {
        loop {
            println!();
            self.fetch_instruction();
            println!();
            match self.execute_instruction() {
                Ok(ExecuteResult::Hault) => break,
                Ok(ExecuteResult::Continue) => continue,
                Err(()) => {
                    break;
                }
            }
        }
    }

    pub fn memory(&self) -> &[u8] {
        &self.memory
    }

    /// Loads the next instruction into ir and advances pc
    fn fetch_instruction(&mut self) {
        self.ir = self.fetch_8_pc();
    }

    /// Reads a byte by loading the address pointed to by pc and increments pc
    fn fetch_8_pc(&mut self) -> u8 {
        let pc = self.pc;
        let a = self.memory[self.pc as usize];
        println!("fetched 8 bits: 0x{a:X} from current pc: 0x{:X}", pc);
        self.pc += 1;
        a
    }

    /// Reads the next 16 bits as a big endian unsigned integer after the current pc, advancing it
    /// by 2 bytes
    fn fetch_16_pc(&mut self) -> u16 {
        let pc = self.pc;
        let a = u16::from_be_bytes([self.memory[pc as usize], self.memory[(pc + 1) as usize]]);
        println!("fetched 16 bits: 0x{a:X} from current pc: 0x{:X}", pc);
        self.pc += 2;
        a
    }

    /// Fetches 8 bits from the given address
    fn fetch_8(&self, addr: u16) -> u8 {
        let a = self.memory[addr as usize];
        println!("fetched 8 bits: 0x{a:X} from [0x{addr:X}]");
        a
    }

    /// Fetches 16 bits from the given address
    fn fetch_16(&self, addr: u16) -> u16 {
        let high = self.memory[addr as usize];
        let low = self.memory[addr as usize + 1];
        let a = u16::from_be_bytes([high, low]);
        println!("fetched 16 bits: 0x{a:X} from [0x{addr:X}]");
        a
    }

    /// Stores `value` into `addr`
    fn store_8(&mut self, addr: u16, value: u8) {
        println!("storing 8 bits: 0x{value:X} to [0x{addr:X}]");
        self.memory[addr as usize] = value;
    }

    /// Stores `value` into `addr`
    fn store_16(&mut self, addr: u16, value: u16) {
        let bytes = value.to_be_bytes();
        println!("storing 16 bits: {value:X} to [{addr:X}]");
        self.memory[addr as usize + 0] = bytes[0];
        self.memory[addr as usize + 1] = bytes[1];
    }

    fn execute_instruction(&mut self) -> Result<ExecuteResult, ()> {
        println!();
        println!("REGISTERS:");
        println!("PC: 0x{:X}", self.pc);
        println!("IR: 0x{:X}", self.ir);
        println!("ACC: 0x{:X}", self.acc);
        println!("MAR: 0x{:X}", self.mar);

        match try_parse(self.ir) {
            None => {
                println!(
                    "illegal instruction: 0b{:08b} at PC: {:X}",
                    self.ir, self.pc
                );
                return Err(());
            }
            Some(ins) => {
                dbg!(&ins);
                match ins {
                    Instruction::Mathmatical { func, src, dst } => {
                        // Holds the address we most recently fetched from
                        let a = match src {
                            SrcTarget::Indirect => self.fetch_8(self.mar) as u16,
                            SrcTarget::Acc => self.acc as u16,
                            SrcTarget::Constant => match dst {
                                // We are storing to an 8 bit register, so only load 16 bits if we
                                // store 16 bits
                                DstTarget::Acc => self.fetch_8_pc() as u16,
                                _ => self.fetch_16_pc(),
                            },
                            SrcTarget::Memory => {
                                let addr = self.fetch_16_pc();
                                self.fetch_16(addr)
                            }
                        };
                        // Gets the second opperand for math as well as an address for write back
                        // if the destination is not a register
                        let (b, addr) = match dst {
                            DstTarget::Indirect => (self.fetch_8(self.mar) as u16, Some(self.mar)),
                            DstTarget::Acc => (self.acc as u16, None),
                            DstTarget::Mar => (self.mar as u16, None),
                            DstTarget::Memory => {
                                let addr = self.fetch_16_pc();
                                (self.fetch_16(addr), Some(addr))
                            }
                        };
                        dbg!(a, b, addr);
                        let result = match func {
                            MathFunction::And => a & b,
                            MathFunction::Or => a | b,
                            MathFunction::Xor => a ^ b,
                            MathFunction::Add => a.wrapping_add(b),
                            MathFunction::Sub => a.wrapping_sub(b),
                            MathFunction::Inc => a.wrapping_add(1),
                            MathFunction::Dec => a.wrapping_sub(1),
                            MathFunction::Not => !a,
                        };
                        dbg!(result);
                        match dst {
                            DstTarget::Indirect => {
                                self.store_8(addr.unwrap(), result as u8);
                                panic!();
                            }
                            DstTarget::Acc => {
                                self.acc = result as u8;
                                println!("storing {} into ACC", self.acc);
                            }
                            DstTarget::Mar => {
                                self.mar = result as u16;
                                println!("storing {} into MAR", self.mar);
                            }
                            DstTarget::Memory => {
                                panic!();
                                self.store_16(addr.unwrap(), result);
                            }
                        }
                    }
                    Instruction::Load { dst, src } => {
                        match src {
                            MemoryMethod::Address => {
                                let addr = dbg!(self.fetch_16_pc());
                                match dst {
                                    Register::Acc => self.acc = dbg!(self.fetch_8(addr)),
                                    Register::Mar => self.mar = dbg!(self.fetch_16(addr)),
                                }
                            }
                            MemoryMethod::Constant => match dst {
                                Register::Acc => self.acc = dbg!(self.fetch_8_pc()),
                                Register::Mar => self.mar = dbg!(self.fetch_16_pc()),
                            },
                            MemoryMethod::Indirect => match dst {
                                Register::Acc => self.acc = dbg!(self.fetch_8(self.mar)),
                                Register::Mar => self.mar = dbg!(self.fetch_16(self.mar)),
                            },
                        };
                    }
                    Instruction::Store { src, dst } => {
                        dbg!(src, dst);
                        match dst {
                            MemoryMethod::Address => {
                                let addr = dbg!(self.fetch_16_pc());
                                match src {
                                    Register::Acc => self.store_8(addr, self.acc as u8),
                                    Register::Mar => self.store_16(addr, self.mar),
                                }
                            }
                            MemoryMethod::Constant => {
                                let addr = self.fetch_16_pc();
                                match src {
                                    Register::Acc => self.store_8(addr, self.acc as u8),
                                    Register::Mar => self.store_16(addr, self.mar),
                                }
                            }
                            MemoryMethod::Indirect => match src {
                                Register::Acc => self.store_8(self.mar, self.acc as u8),
                                Register::Mar => self.store_16(self.mar, self.mar),
                            },
                        };
                    }
                    Instruction::Branch(kind) => {
                        let jmp_addr = self.fetch_16_pc();
                        match kind {
                            BranchKind::Bra => {
                                self.pc = jmp_addr;
                            }
                            BranchKind::Brz => {
                                if self.acc == 0 {
                                    self.pc = jmp_addr;
                                }
                            }
                            BranchKind::Bne => {
                                if self.acc != 0 {
                                    self.pc = jmp_addr;
                                }
                            }
                            BranchKind::Blt => {
                                if self.acc < 0 {
                                    self.pc = jmp_addr;
                                }
                            }
                            BranchKind::Ble => {
                                if self.acc <= 0 {
                                    self.pc = jmp_addr;
                                }
                            }
                            BranchKind::Bgt => {
                                if self.acc > 0 {
                                    self.pc = jmp_addr;
                                }
                            }
                            BranchKind::Bge => {
                                if self.acc >= 0 {
                                    self.pc = jmp_addr;
                                }
                            }
                        }
                    }
                    Instruction::Nop => {}
                    Instruction::Hault => {
                        return Ok(ExecuteResult::Hault);
                    }
                }
            }
        }
        return Ok(ExecuteResult::Continue);
    }
}
