
use std::{fs, path::Path, str::Lines, collections::HashMap};


fn main() {
    let input_path: &Path =Path::new("./message_04.txt");
    let input: String = fs::read_to_string(input_path).unwrap();
    let real_checksums = get_real_files(input.lines());
    println!("{:?}", real_checksums[32]);
}

fn get_real_files(lines: Lines) -> Vec<String> {
    let mut result = Vec::new();
    for line in lines {
        if let [name, checksum] = line.split('-').collect::<Vec<&str>>()[0..2] {
            if is_valid_checksum(name, &checksum) {
                result.push(checksum.to_string());
            }
        }
    }
    result
}

//O(3n) -> n = chars in the file name
fn is_valid_checksum(name: &str, checksum: &str) -> bool {
    let mut chars_dict: HashMap<char, u8> = HashMap::new();
    let mut order: Vec<char> = Vec::new();
    let mut correct_checksum = String::new();

    //O(n)
    for char in name.chars() {
        match chars_dict.get(&char) {
            Some(val) => chars_dict.insert(char, val + 1),
            None => {
                order.push(char);
                chars_dict.insert(char, 1)
            },
        };
    }

    //O(n)
    for char in order {
        if let Some(1) = chars_dict.get(&char) {
            correct_checksum.push(char);
        }
    }
    
    //O(n)
    return correct_checksum == checksum;
}
