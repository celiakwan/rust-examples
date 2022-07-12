use std::{collections::HashMap, thread};

pub fn frequency(input: &Vec<String>, worker_count: usize) -> HashMap<char, usize> {
    let mut result = HashMap::<char, usize>::new();
    let chunks = input.chunks((input.len() / worker_count).max(1));
    let mut handles = Vec::new();

    for chunk in chunks {
        let text = chunk.join("");
        let handle = thread::spawn(move || {
            let mut map = HashMap::<char, usize>::new();
            for c in text.chars().filter(|c| c.is_alphabetic()) {
                *map.entry(c.to_ascii_lowercase()).or_default() += 1;
            }
            map
        });
        handles.push(handle);
    }

    for handle in handles {
        let map = handle.join().unwrap();
        for (key, value) in map {
            *result.entry(key).or_default() += value;
        }
    }
    
    result
}
