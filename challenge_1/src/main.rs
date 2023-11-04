use std::{fs, path, collections::HashMap};

fn main() {
    let input_path = path::Path::new("./message_01.txt");
    let input = fs::read_to_string(input_path).unwrap();

    let vec_of_words = split_input(input);
    let hash_map = words_to_hashmap(&vec_of_words);
    let result_string = get_string_result(vec_of_words, hash_map);
    println!("{:?}", result_string);
}

fn split_input(input: String) -> Vec<String> {
    input.as_str()
        .split_whitespace()
        .map(str::to_lowercase)
        .collect()
}

fn words_to_hashmap(words: &Vec<String>) -> HashMap<String, i32> {
    let mut result: HashMap<String, i32> = HashMap::new();
    for word in words {
        let new_value;
        match result.get(&word.to_lowercase()) {
            None => new_value = 1,
            Some(current_value) => new_value = current_value + 1
        }
        result.insert(String::from(word.to_lowercase()), new_value);
    }
    result
}

fn get_string_result(words:Vec<String>, mut hm: HashMap<String, i32>) -> String {
    let mut final_string = String::from("");
    for word in words {
        match hm.remove(&word) {
            Some(val) => {
                final_string.push_str(word.as_str());
                final_string.push_str(val.to_string().as_str())
            },
            None => {}
        }
    }

    final_string
}