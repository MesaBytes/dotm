// manage config file

use std::collections::HashMap;

pub struct Config {
    file_path: String,
    map: HashMap<String, String>,
}

impl Config {
    pub fn new(file_path: &str) -> Self {
        if std::path::Path::new(file_path).exists() == false {
            std::fs::write(file_path, "").expect("Failed to write file");
        }

        let mut map = HashMap::new();
        let contents = std::fs::read_to_string(file_path).expect("Failed to read file");

        for line in contents.lines() {
            if line.starts_with('#') || line.starts_with("//"){
                continue;
            }

            let (key, value) = line.split_once('=').expect("Corrupt config file!");
            map.insert(key.to_owned(), value.to_owned());
        }

        Self {
            file_path: file_path.to_owned(),
            map,
        }
    }

    pub fn insert(&mut self, key: String, value: String) -> () {
        self.map.insert(key, value);
        self.save();
    }

    pub fn get(&self, key: &str) -> &str {
        match self.map.get(key) {
            Some(val) => val,
            None => "",
        }
    }

    fn save(&self) -> () {
        let mut contents = String::new();
        for (key, value) in &self.map {
            contents.push_str(key);
            contents.push('=');
            contents.push_str(value);
            contents.push('\n');
        }

        std::fs::write(self.file_path.clone(), contents).expect("Failed to write file");
    }
}
