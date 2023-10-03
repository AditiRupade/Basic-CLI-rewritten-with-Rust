use std::env;

fn join_arguments(args: Vec<String>) -> String {
    args[1..].join(" ")
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Call the function to join all command line arguments
    let message = join_arguments(args);
    println!("{}", message);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_join_arguments() {
        let args = vec![
            "program_name".to_string(),
            "Hello".to_string(),
            "world".to_string(),
        ];
        let result = join_arguments(args);
        assert_eq!(result, "Hello world");
    }

    #[test]
    fn test_join_arguments_empty() {
        let args = vec!["program_name".to_string()];
        let result = join_arguments(args);
        assert_eq!(result, "");
    }
}