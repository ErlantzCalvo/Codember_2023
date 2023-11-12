use std::{fs, path::{self, Path}, str::Split};

fn main() {
    let input_path: &Path = path::Path::new("./message_02.txt");
    let input: String = fs::read_to_string(input_path).unwrap();
    let chars: Split<&str> = input.as_str().split("");
    let result: String = decode_string(chars);

    println!("{:?}",  result);
}

fn decode_string(chars:Split<&str>) -> String {
    let mut result_string: String = String::new();
    let mut current_number:i32 = 0;
    for char in chars {
        match char {
            "#" => current_number += 1,
            "@" => current_number -= 1,
            "*" => current_number *= current_number,
            "&" => result_string.push_str(&current_number.to_string()),
            _ => {}
        }
    }
    result_string
}
