
use std::{fs, path::Path, str::Lines};
use regex::Regex;

fn main() {
    let input_path: &Path =Path::new("./message_05.txt");
    let input: String = fs::read_to_string(input_path).unwrap();
    let secret = solve(input.lines());
    println!("{:?}", secret);
}

fn solve(lines: Lines) -> String {
    let mut secret = String::new();
    for line in lines {
        if let [id, username, email, age, location] = line.split(',').collect::<Vec<&str>>()[0..5] {
            if (id.len() == 0 || !id.chars().all(char::is_alphanumeric)) ||
                (username.len() == 0 || !username.chars().all(char::is_alphanumeric)) || 
                (!is_valid_email(email)) || (age.len() > 0 && !age.chars().all(char::is_numeric)) ||
                (location.len() > 0 && !location.replace(" ","").chars().all(char::is_alphabetic)) {

                secret.push(username.chars().nth(0).unwrap())   
            }
        }
        
    }
    secret
}

fn is_valid_email(email: &str) -> bool {
    let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap(); 
    email_regex.is_match(email)
}
