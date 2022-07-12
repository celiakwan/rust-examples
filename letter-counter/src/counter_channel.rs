use std::{collections::HashMap, mem, sync::mpsc, thread};

pub fn frequency(input: &Vec<String>, worker_count: usize) -> HashMap<char, usize> {
    let mut result = HashMap::<char, usize>::new();
    let chunks = input.chunks((input.len() / worker_count).max(1));
    let (sender, receiver) = mpsc::channel();

    for chunk in chunks {
        let sender = sender.clone();
        let text = chunk.join("");
        thread::spawn(move || {
            let mut map = HashMap::<char, usize>::new();
            for c in text.chars().filter(|c| c.is_alphabetic()) {
                *map.entry(c.to_ascii_lowercase()).or_default() += 1;
            }
            sender.send(map).unwrap();
        });
    }

    mem::drop(sender);

    for received in receiver {
        for (key, value) in received {
            *result.entry(key).or_default() += value;
        }
    }

    result
}
