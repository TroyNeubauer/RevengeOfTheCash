pub const MEMORY_SIZE: usize = 64 * 1024;

pub struct Computer {
    memory: [u8; MEMORY_SIZE],
    acc: u8,
    ir: u8,
    mar: u32,
    pc: usize,
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

    fn fetch_instruction(&self) {}

    fn execute_instruction(&self) -> ExecuteResult {
        return ExecuteResult::Hault;
    }
}
