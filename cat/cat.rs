use std::env;
use std::fs::File;
use std::io::{Read};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Error: Unable to open file '{}'", filename);
            std::process::exit(1);
        }
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {
            print!("{}", contents);
        }
        Err(_) => {
            eprintln!("Error: Unable to read file '{}'", filename);
            std::process::exit(1);
        }
    }
}
