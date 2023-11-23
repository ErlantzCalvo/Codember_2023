
use std::{fs, path::{self, Path}};


fn main() {
    let input_path: &Path = path::Path::new("./message_03.txt");
    let input: String = fs::read_to_string(input_path).unwrap();
    let invalid_passwords = check_all_passwords(input);

    println!("Incorrect 13 --> {:?}", invalid_passwords[12]);
    println!("Incorrect 42 --> {:?}", invalid_passwords[41]);
}

fn check_all_passwords(input: String) -> Vec<String> {
    let mut invalid_passwords = vec![];
    let lines = input.lines();
    for line in lines {
        if !is_valid_pass(line) {
            invalid_passwords.push(line.to_string());
        }
    }

    invalid_passwords
}

fn is_valid_pass(input: &str) -> bool {
    let split: Vec<&str> = input.split(['-',' ',':']).collect();
    
    if let [min_str, max_str, letter, _, pass] = &split[..] {
        let min: usize = min_str.parse().unwrap();
        let max: usize = max_str.parse().unwrap();
        let required_char = letter.chars().next().unwrap();
    
        if min > pass.len() {
            return false;
        } 

        let mut count = 0;
        pass.chars().for_each(|c| {
            if c == required_char {
                count += 1;
            }
        });

        return count >= min && count <= max;
    }

    false
}
