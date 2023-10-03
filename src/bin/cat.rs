use std::env;
use std::fs::File;
use std::io::{Read};

fn read_file(filename: &str) -> Result<String, String> {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => {
            return Err(format!("Error: Unable to open file '{}'", filename));
        }
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {
            Ok(contents)
        }
        Err(_) => {
            Err(format!("Error: Unable to read file '{}'", filename))
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];

    match read_file(filename) {
        Ok(contents) => {
            print!("{}", contents);
        }
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file_success() {
        let content = read_file("myfile.txt");
        assert!(content.is_ok());
        assert_eq!(content.unwrap(), "Hello, World!\nGetting started with Rust\nBye");
    }

    #[test]
    fn test_read_file_failure() {
        let content = read_file("nonexistent_file.txt");
        assert!(content.is_err());
        assert_eq!(content.err().unwrap(), "Error: Unable to open file 'nonexistent_file.txt'");
    }
}
