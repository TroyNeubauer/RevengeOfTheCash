mod computer;
mod instruction;
mod parser;

pub use computer::*;
pub use instruction::*;
pub use parser::*;

fn main() {
    let mut memory = [0u8; MEMORY_SIZE];
    let initial_memory = include!("../mem_in.txt");
    (&mut memory[..initial_memory.len()]).copy_from_slice(&initial_memory);

    let mut computer = Computer::new(memory);
    computer.run();
    let expected_memory = include!("../mem_out.txt");
    pretty_assertions::assert_eq!(computer.memory(), expected_memory);
}
