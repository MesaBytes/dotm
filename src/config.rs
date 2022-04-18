// manage config file

use std::collections::HashMap;

pub struct Config {
    file_path: String,
    map: HashMap<String, String>,
}

impl Config {
    pub fn new(file_path: &str) -> Result<Config, std::io::Error> {
        if std::path::Path::new(file_path).exists() == false {
            std::fs::write(file_path, "")?;
        }

        let mut map = HashMap::new();
        let contents = std::fs::read_to_string(file_path)?;

        for line in contents.lines() {
            let (key, value) = line.split_once('=').expect("corrupt database!");
            map.insert(key.to_owned(), value.to_owned());
        }

        Ok(Config {
            file_path: file_path.to_owned(),
            map,
        })
    }

    pub fn insert(&mut self, key: String, value: String) -> Result<(), std::io::Error> {
        self.map.insert(key, value);
        self.save()?;
        Ok(())
    }

    pub fn get(&self, key: &str) -> &String {
        return self.map.get(key).unwrap();
    }

    fn save(&self) -> Result<(), std::io::Error> {
        let mut contents = String::new();
        for (key, value) in &self.map {
            contents.push_str(key);
            contents.push('=');
            contents.push_str(value);
            contents.push('\n');
        }

        std::fs::write(self.file_path.clone(), contents)?;
        Ok(())
    }
}
