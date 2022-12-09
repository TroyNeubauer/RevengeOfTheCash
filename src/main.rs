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
    std::fs::write("mem_in.bin", initial_memory).unwrap();

    let mut computer = Computer::new(memory);
    computer.run();

    let expected_memory = include!("../mem_out.txt");
    let actual_memory = computer.memory();
    std::fs::write("mem_out.bin", actual_memory).unwrap();
    for i in 0..expected_memory.len() {
        let expected = expected_memory[i];
        let actual = actual_memory[i];
        if expected != actual {
            println!("{i:X} differs expected {expected:X}, was {actual:}");
        }
    }
}
