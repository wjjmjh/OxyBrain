use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("error: No file path provided.\nPlease specify the path to a Brainfuck file.\nUsage: oxybrain <path_to_brainfuck_file>");
        return;
    }

    let file_path = &args[1];

    if !Path::new(file_path).exists() {
        eprintln!("error: the brainfuck file does not exist.");
        return;
    }

    let code = fs::read_to_string(file_path).expect("error reading the brainfuck file");

    println!("got brainfuck code:\n{}", code);
}
