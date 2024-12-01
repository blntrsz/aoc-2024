pub mod common {
    use std::fs;

    pub fn read_file(file_name: &str) -> String {
        match fs::read_to_string(file_name) {
            Ok(content) => content,
            Err(error) => {
                eprintln!("Error reading file {}: {}", file_name, error); // Print the error
                String::new()
            }
        }
    }

    pub fn string_to_lines(file: String) -> Vec<String> {
        file.lines().map(|line| line.to_string()).collect()
    }
}
