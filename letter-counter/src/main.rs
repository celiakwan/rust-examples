use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

mod counter_channel;
mod counter_joinhandle;
mod counter_mutex;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Missing arguments");
    }

    let file = File::open("test.txt").expect("File not found");
    let reader = BufReader::new(file);
    let input = reader.lines().collect::<Result<Vec<String>, _>>().unwrap();

    let result = match args[1].as_str() {
        "joinhandle" => counter_joinhandle::frequency(&input, 3),
        "channel" => counter_channel::frequency(&input, 3),
        "mutex" => counter_mutex::frequency(&input, 3),
        _ => HashMap::new(),
    };
    print!("Result: {:?}", result);
}
