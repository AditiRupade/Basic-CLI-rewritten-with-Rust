use std::env;
use std::fs;

fn main() {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the user provided the directory name
    if args.len() != 2 {
        eprintln!("Usage: {} <directory_name>", args[0]);
        std::process::exit(1);
    }

    // Parse the command-line argument as the directory name
    let dir_name = &args[1];

    // Create the directory
    if let Err(err) = fs::create_dir(dir_name) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    } else {
        println!("Directory '{}' created successfully.", dir_name);
    }
}
