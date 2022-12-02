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
                    // TODO
                }
                Instruction::Load { register, method } => {
                    // TODO
                }
                Instruction::Store { register, method } => {
                    // TODO
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
                    // TODO
                }
                Instruction::Hault => {
                    return ExecuteResult::Hault;
                }
            },
        }
        return ExecuteResult::Continue;
    }

    /// Reads a byte by loading the address pointed to by pc and increments pc
    fn load_pc(&mut self) -> u8 {
        let byte = self.memory[self.pc as usize];
        self.pc += 1;
        byte
    }
}