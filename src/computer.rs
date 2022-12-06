use std::str::pattern::Searcher;
use crate::{try_parse, Instruction, MemoryMethod, Register};
use crate::*;

pub const MEMORY_SIZE: usize = 64 * 1024;

pub struct Computer {
    memory: [u8; MEMORY_SIZE],
    acc: i8,
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
            self.fetch_instruction();
            if self.execute_instruction() == ExecuteResult::Hault {
                break;
            }
        }
    }

    fn fetch_instruction(&mut self) {
        self.ir = self.load_pc();
    }

	fn fetch_16_bits(&mut self) {
		
        self.pc = ((self.memory[self.pc as usize] as u16) << 8 + 			 self.memory[(self.pc+1) as usize]) as u16;
    }

    fn execute_instruction(&mut self) -> ExecuteResult {
        match try_parse(self.ir) {
            None => panic!("illegal instruction: 0b{:b}", self.ir),
            Some(ins) => match ins {
                Instruction::Mathmatical { func, src, dst } => {
                    let a_value = self.load_target(src);
                    let b_value = self.load_target(dst);
                    let dst_value = match func {
                        MathFunction::Add => a_value.value + b_value.value,
                        _ => panic!(),
                    };
                    MathFunction::Sub => a_value.value - b_value.value,
                        _ => panic!(),
                    };
                    MathFunction::Xor => a_value.value ^ b_value.value,
                    _ => panic!(),
                    };
                    MathFunction::And => a_value.value & b_value.value,
                    _ => panic!(),
                    };
                    MathFunction::Or => a_value.value | b_value.value,
                    _ => panic!(),
                    };
                    MathFunction::Inc => a_value.value++,
                    _ => panic!(),
                    };
                    MathFunction::Dec => a_value.value--,
                    _ => panic!(),
                    };
                    MathFunction::Not => ~a_value.value,
                    _ => panic!(),
                    };
                }
                Instruction::Load { register, method } => {
                    // TODO
                }
                Instruction::Store { register, method } => {
                    //TODO
                }   
                Instruction::Branch(kind) => {
                    // TODO
					match kind {
						BranchKind::Bra => {
							self.fetch_16_bits();
						}
						BranchKind::Brz => {
							if self.acc == 0 {
								self.fetch_16_bits();
							}
						}
						BranchKind::Bne => {
							if self.acc != 0 {
								self.fetch_16_bits();
							}
						}
						BranchKind::Blt => {
							if self.acc < 0 {
								self.fetch_16_bits();
							}
						}
						BranchKind::Ble => {
							if self.acc <= 0 {
								self.fetch_16_bits();
							}
						}
						BranchKind::Bgt => {
							if self.acc > 0 {
								self.fetch_16_bits();
							}
						}
						BranchKind::Bge => {
							if self.acc >= 0 {
								self.fetch_16_bits();
							}
						}
					}
						
                }
                Instruction::Nop => {
                }
                Instruction::Hault => {
                    return ExecuteResult::Hault;
                }
            },
        }
        return ExecuteResult::Continue;
    }

        /// Loads the value referenced by target into a target ref containing the value and the address
    /// used to load the value (if present).
    fn load_target(&mut self, target: Target) -> TargetRef {
        match target {
            Target::Indirect => TargetRef::immediate(self.load_pc()),
            Target::Acc => TargetRef {
                value: self.acc,
                addr: None,
            },
            Target::Mar => self.load_target_via_address(self.mar),
            Target::Memory => {
                let addr = (self.load_pc() as u16) >> 8 | self.load_pc() as u16;
                self.load_target_via_address(addr)
            }
        }
    }

    fn load_target_via_address(&self, addr: u16) -> TargetRef {
        TargetRef {
            value: self.memory[addr as usize],
            addr: Some(addr),
        }
    }

    /// Stores the
    fn store_target(&mut self, addr: Option<u16>, value: u8) {
        if let Some(addr) = addr {
            self.memory[addr as usize] = value;
        } else {
            panic!("Can not store with a non memory operand");
        }
    }

    /// Reads a byte by loading the address pointed to by pc and increments pc
    fn load_pc(&mut self) -> u8 {
        let byte = self.memory[self.pc as usize];
        self.pc += 1;
        byte
    }
}