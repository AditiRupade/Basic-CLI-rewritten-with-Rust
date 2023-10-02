use std::env;
use std::fs;
use std::path::Path;

fn remove_file_or_dir(path: &Path) -> std::io::Result<()> {
    if path.is_file() {
        fs::remove_file(path)?;
    } else if path.is_dir() {
        fs::remove_dir_all(path)?;
    }
    Ok(())
}

fn main() {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the user provided at least one path to remove
    if args.len() < 2 {
        eprintln!("Usage: {} <file_or_directory>...", args[0]);
        std::process::exit(1);
    }

    // Iterate over the provided paths and attempt to remove them
    for path_str in &args[1..] {
        let path = Path::new(path_str);
        if let Err(err) = remove_file_or_dir(&path) {
            eprintln!("Error: {}", err);
        } else {
            println!("Removed: {}", path.display());
        }
    }
}
