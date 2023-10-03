use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn head(filename: &str, num_lines: usize) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut lines = Vec::new();

    for (line_num, line) in reader.lines().enumerate() {
        if line_num >= num_lines {
            break;
        }

        if let Ok(line) = line {
            lines.push(line);
        }
    }

    Ok(lines)
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
    match head(filename, num_lines) {
        Ok(lines) => {
            for line in lines {
                println!("{}", line);
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_head_success() {
        let filename = "test.txt";

        let result = head(filename, 3);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec!["Line 1", "Line 2", "Line 3"]);
    }

    #[test]
    fn test_head_file_not_found() {
        let filename = "nonexistent_file.txt";
        let result = head(filename, 3);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().kind(), io::ErrorKind::NotFound);
    }
}
