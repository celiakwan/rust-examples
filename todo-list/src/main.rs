use std::collections::HashMap;
// use std::io::Read;

fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item");
    let mut todo = Todo::new().expect("Initialisation failed");
    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("A to-do item has been added"),
            Err(e) => println!("Error occurred: {}", e),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            Some(_) => match todo.save() {
                Ok(_) => println!("The to-do item has been updated"),
                Err(e) => println!("Error occurred: {}", e),
            },
            None => println!("'{}' is not present in the list", item),
        }
    }
}

struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    fn new() -> Result<Todo, std::io::Error> {
        let f = std::fs::OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open("records.json")?;
        match serde_json::from_reader(f) {
            Ok(map) => Ok(Todo { map }),
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            Err(e) => panic!("Error occurred: {}", e),
        }
    }
    // fn new() -> Result<Todo, std::io::Error> {
    //     let mut f = std::fs::OpenOptions::new().create(true).read(true).write(true).open("records.txt")?;
    //     let mut content = String::new();
    //     f.read_to_string(&mut content)?;
    //     let map: HashMap<String, bool> = content
    //     .lines()
    //     .map(|line| line.splitn(2, ", ").collect::<Vec<&str>>())
    //     .map(|v| (v[0], v[1]))
    //     .map(|(k, v)| (String::from(k), std::str::FromStr::from_str(v).expect("Could not parse the string to bool")))
    //     .collect();
    //     Ok(Todo { map })
    // }

    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }

    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        let f = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open("records.json")?;
        serde_json::to_writer_pretty(f, &self.map)?;
        Ok(())
    }
    // fn save(self) -> Result<(), std::io::Error> {
    //     let mut content = String::new();
    //     for (k, v) in self.map {
    //         let record = format!("{}, {}\n", k, v);
    //         content.push_str(&record);
    //     }
    //     std::fs::write("records.txt", content)
    // }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }
}
