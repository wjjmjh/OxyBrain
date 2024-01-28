use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::Path;

const MEMORY_SIZE: usize = 30000; // conventional memory limit

fn interpret(code: &str) {
    let mut memory = [0u8; MEMORY_SIZE];
    let mut pointer = 0;
    let mut loop_stack = Vec::new();
    let mut program_counter = 0;

    while program_counter < code.len() {
        match code.chars().nth(program_counter).unwrap() {
            '>' => pointer = (pointer + 1) % MEMORY_SIZE, // move the pointer to the right 1 block, with circular memory behavior
            '<' => pointer = (pointer + MEMORY_SIZE - 1) % MEMORY_SIZE, // move the pointer to the left 1 block, with circular memory behavior
            '+' => memory[pointer] = memory[pointer].wrapping_add(1), // increase value stored at the block pointed to by the memory pointer
            '-' => memory[pointer] = memory[pointer].wrapping_sub(1), // decrease value stored at the block pointed to by the memory pointer
            '.' => print!("{}", memory[pointer] as char), // print 1 character to the console
            ',' => {
                let mut input = [0]; // to store the character input read from the user
                io::stdin().read_exact(&mut input).unwrap(); // read one byte of input from the standard input
                memory[pointer] = input[0]; // assign it to the memory cell currently pointed to by the pointer
            } // input 1 character
            '[' => {
                // loop should be skipped
                if memory[pointer] == 0 {
                    // keep track of the end of the loop
                    let mut loop_end = 1;
                    // skip over the loop and find its end
                    while loop_end > 0 {
                        program_counter += 1;
                        match code.chars().nth(program_counter).unwrap() {
                            '[' => loop_end += 1, // another nested loop is encountered within the loop
                            ']' => loop_end -= 1, // the end of the loop is reached,  counting down the nested loops
                            _ => {}               // ignored chars
                        }
                    }
                } else {
                    loop_stack.push(program_counter); // the loop should be entered, to remember the starting position of the loop
                }
            }
            ']' => {
                // check if there's a corresponding loop starting position on the loop_stack
                if let Some(start_loop) = loop_stack.pop() {
                    // check if the memory cell is not 0
                    if memory[pointer] != 0 {
                        program_counter = start_loop; // jump back to the start of the loop to repeat it
                        continue;
                    }
                }
            }
            _ => {} // ignored chars
        }
        program_counter += 1; // move to the next position in the code
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("error: no file path provided, please specify the path to a Brainfuck file.\nusage: oxybrain <path_to_brainfuck_file>");
        return;
    }

    let file_path = &args[1];

    if !Path::new(file_path).exists() {
        eprintln!("error: the brainfuck file does not exist.");
        return;
    }

    let code = fs::read_to_string(file_path).expect("error reading the brainfuck file.");

    interpret(&code)
}
