use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn tail(filename: &str, num_lines: usize) -> io::Result<()> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let start_index = if lines.len() > num_lines {
        lines.len() - num_lines
    } else {
        0
    };

    for line in &lines[start_index..] {
        println!("{}", line);
    }

    Ok(())
}

fn main() {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the user provided the filename and number of lines
    if args.len() != 3 {
        eprintln!("Usage: {} <filename> <num_lines>", args[0]);
        std::process::exit(1);
    }

    // Parse command-line arguments
    let filename = &args[1];
    let num_lines = match args[2].parse::<usize>() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: Invalid number of lines.");
            std::process::exit(1);
        }
    };

    // Call the tail function
    if let Err(err) = tail(filename, num_lines) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
