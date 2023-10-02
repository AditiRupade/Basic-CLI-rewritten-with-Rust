use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <pattern> <file>", args[0]);
        std::process::exit(1);
    }

    let pattern = &args[1];
    let filename = &args[2];
    let file = match File::open(filename) {
        Ok(file) => BufReader::new(file),
        Err(_) => {
            eprintln!("Error: Unable to open file '{}'", filename);
            std::process::exit(1);
        }
    };

    for line in file.lines() {
        if let Ok(line) = line {
            if line.contains(pattern) {
                println!("{}", line);
            }
        }
    }
}
