use std::env;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;

fn touch_file(path: &Path) -> std::io::Result<()> {
    match fs::File::create(path) {
        Ok(_) => Ok(()),
        Err(e) if e.kind() == ErrorKind::AlreadyExists => {
            Err(std::io::Error::new(
                ErrorKind::Other,
                format!("Error: File already exists: {}", path.display()),
            ))
        }
        Err(e) => Err(e),
    }
}

fn main() {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the user provided at least one file to touch
    if args.len() < 2 {
        eprintln!("Usage: {} <file>...", args[0]);
        std::process::exit(1);
    }

    // Iterate over the provided file paths and touch each one
    for path_str in &args[1..] {
        let path = Path::new(path_str);
        match touch_file(&path) {
            Ok(_) => println!("Touched: {}", path.display()),
            Err(err) => eprintln!("{}", err),
        }
    }
}
