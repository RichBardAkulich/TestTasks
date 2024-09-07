use std::io;
use std::fs;
use std::error::Error;
use std::process;

struct Text {
    text: String
}

impl Text {
    fn num_chars(&self) -> usize {
        let mut num_chars = 0;
        for char in self.text.chars() {
            num_chars += 1;
        }
        return num_chars
    }
    fn num_words(&self) -> i32 {
        let mut num_words = 1;
        for char in self.text.chars() {
            if char == ' ' || char == '\n' {
                num_words += 1;
            }
        }
        return num_words
    }

    fn num_lines(&self) -> i32 {
        let mut num_lines = 1;
        for char in self.text.chars() {
            if char == '\n' {
                num_lines += 1;
            }
        }
        return num_lines
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut file_path: String = Default::default();
    io::stdin().read_line(&mut file_path)
        .ok()
        .expect("Error read line!");
    let len = file_path.len();
    file_path.truncate(len - 2);
    let mut file_contents: String = Default::default();
    match fs::metadata(&file_path) {
        Ok(metadata) => {
            if metadata.is_file() {
                file_contents = fs::read_to_string(&file_path)?;
            } else {
                println!("Path exists, but it's not a file.");
                process::exit(1);
            }
        }
        Err(_) => {
            println!("File does not exist.");
            process::exit(1);
        }
    }
    let text = Text { text: file_contents };
    let num_chars = text.num_chars();
    let num_words = text.num_words();
    let num_lines = text.num_lines();
    println!("Words: {}", num_words);
    println!("Lines: {}", num_lines);
    println!("Characters: {}", num_chars);

    Ok(())
}