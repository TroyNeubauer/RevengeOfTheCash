use crate::*;
use crate::{try_parse, Instruction, MathFunction, Target};

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

    pub fn memory(&self) -> &[u8] {
        &self.memory
    }

    /// Loads the next instruction into ir and advances pc
    fn fetch_instruction(&mut self) {
        self.ir = self.fetch_8_pc();
    }

    /// Reads a byte by loading the address pointed to by pc and increments pc
    fn fetch_8_pc(&mut self) -> u8 {
        let byte = self.memory[self.pc as usize];
        self.pc += 1;
        byte
    }

    /// Reads the next 16 bits as a big endian unsigned integer after the current pc, advancing it
    /// by 2 bytes
    fn fetch_16_pc(&mut self) -> u16 {
        u16::from_be_bytes([self.fetch_8_pc(), self.fetch_8_pc()])
    }

    /// Fetches 8 bits from the given address
    fn fetch_8(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    /// Fetches 16 bits from the given address
    fn fetch_16(&self, addr: u16) -> u16 {
        u16::from_be_bytes([self.fetch_8(addr), self.fetch_8(addr + 1)])
    }

    /// Stores `value` into `addr`
    fn store_8(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }

    /// Stores `value` into `addr`
    fn store_16(&mut self, addr: u16, value: u16) {
        let bytes = value.to_be_bytes();
        self.memory[addr as usize + 0] = bytes[0];
        self.memory[addr as usize + 1] = bytes[1];
    }

    fn execute_instruction(&mut self) -> ExecuteResult {
        println!("executing {:b}", self.ir);
        match try_parse(self.ir) {
            None => panic!("illegal instruction: 0b{:08b}", self.ir),
            Some(ins) => match ins {
                Instruction::Mathmatical { func, src, dst } => {
                    let a_value = self.load_target(src);
                    let b_value = self.load_target(dst);
                    dbg!(a_value, b_value, func);
                    let dst_value = match func {
                        MathFunction::And => a_value.value & b_value.value,
                        MathFunction::Or => a_value.value | b_value.value,
                        MathFunction::Xor => a_value.value ^ b_value.value,
                        MathFunction::Add => a_value.value + b_value.value,
                        MathFunction::Sub => a_value.value - b_value.value,
                        MathFunction::Inc => a_value.value + 1,
                        MathFunction::Dec => a_value.value - 1,
                        MathFunction::Not => !a_value.value,
                    };
                    self.store_target(a_value.addr, dst_value);
                }
                Instruction::Load { dst, src } => {
                    dbg!(src, dst);
                    match src {
                        MemoryMethod::Address => {
                            let addr = self.fetch_16_pc();
                            match dst {
                                Register::Acc => self.acc = dbg!(self.fetch_8(addr) as i8),
                                Register::Mar => self.mar = dbg!(self.fetch_16(addr)),
                            }
                        }
                        MemoryMethod::Constant => match dst {
                            Register::Acc => self.acc = self.fetch_8_pc() as i8,
                            Register::Mar => self.mar = self.fetch_16_pc(),
                        },
                        MemoryMethod::Indirect => match dst {
                            Register::Acc => self.acc = self.fetch_8(self.mar) as i8,
                            Register::Mar => self.mar = self.fetch_16(self.mar),
                        },
                    };
                }
                Instruction::Store { src, dst } => {
                    dbg!(src, dst);
                    match dst {
                        MemoryMethod::Address => {
                            let addr = self.fetch_16_pc();
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
                    // TODO
                    match kind {
                        BranchKind::Bra => {
                            self.pc = self.fetch_16_pc();
                        }
                        BranchKind::Brz => {
                            if self.acc == 0 {
                                self.pc = self.fetch_16_pc();
                            }
                        }
                        BranchKind::Bne => {
                            if self.acc != 0 {
                                self.pc = self.fetch_16_pc();
                            }
                        }
                        BranchKind::Blt => {
                            if self.acc < 0 {
                                self.pc = self.fetch_16_pc();
                            }
                        }
                        BranchKind::Ble => {
                            if self.acc <= 0 {
                                self.pc = self.fetch_16_pc();
                            }
                        }
                        BranchKind::Bgt => {
                            if self.acc > 0 {
                                self.pc = self.fetch_16_pc();
                            }
                        }
                        BranchKind::Bge => {
                            if self.acc >= 0 {
                                self.pc = self.fetch_16_pc();
                            }
                        }
                    }
                }
                Instruction::Nop => {}
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
            Target::Indirect => TargetRef::immediate(self.fetch_8_pc()),
            Target::Acc => TargetRef {
                value: self.acc as u8,
                addr: None,
            },
            Target::Mar => self.load_target_via_address(self.mar),
            Target::Memory => {
                let addr = (self.fetch_8_pc() as u16) >> 8 | self.fetch_8_pc() as u16;
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
}

#[derive(Copy, Clone, Debug)]
struct TargetRef {
    value: u8,
    addr: Option<u16>,
}

impl TargetRef {
    pub fn immediate(value: u8) -> TargetRef {
        TargetRef { value, addr: None }
    }
}

