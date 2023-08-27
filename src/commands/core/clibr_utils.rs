pub mod utils {

    use regex::Regex;
    use std::fs::File;
    use std::io::Write;
    use std::io::{Error, Read};

    pub fn version() -> String {
        String::from("0.0.1")
    }

    pub fn get_size_file(path: &str) -> Result<String, Error> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let size = buffer.len();
        Ok(format!("({} bytes)", size))
    }

    pub fn read_from_file(file_path: &str) -> Result<String, Error> {
        let mut file = File::open(file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        Ok(content)
    }

    pub fn write_to_file(file_path: &str, content: &str) -> Result<(), Error> {
        let mut file = File::create(file_path)?;
        file.write_all(content.as_bytes())?;

        Ok(())
    }

    pub fn regex_replace_all(input: &str, pattern: &str, replacement: &str) -> String {
        let regex_pattern = Regex::new(pattern).unwrap();
        regex_pattern.replace_all(input, replacement).to_string()
    }
}
