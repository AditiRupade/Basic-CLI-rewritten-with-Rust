use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn head(filename: &str, num_lines: usize) -> io::Result<()> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for (line_num, line) in reader.lines().enumerate() {
        if line_num >= num_lines {
            break;
        }

        if let Ok(line) = line {
            println!("{}", line);
        }
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

    // Call the head function
    if let Err(err) = head(filename, num_lines) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
