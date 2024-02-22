use serde::{Deserialize, Serialize};
use std::{fs::File, path::Path};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub parsers: Vec<String>,
}

pub fn get_path() -> String {
    let home = home::home_dir().unwrap();
    home.join(".config").join("smart-open.json").to_str().unwrap().to_string()
}

pub fn read() -> Config {
    let config_path = get_path();
    let path = Path::new(&config_path);
    
    if !path.exists() {
        let file = File::create(&config_path).expect("Failed to create config file");

        let default_config = Config {
            parsers: vec![
                String::from("url"),
                String::from("git"), 
                String::from("file"), 
            ]
        };

        serde_json::to_writer_pretty(file, &default_config).expect("Failed to write to config file");
    }

    let file = File::open(path).expect("Failed to open config file");
    
    serde_json::from_reader::<File, Config>(file).expect("Failed to parse config file")
}

#[cfg(test)]
mod tests {
    use crate::utils::config::get_path;

    #[test]
    fn get_path_should_return_config_directory() {
        let home = home::home_dir().unwrap();
        let path = home.join(".config").join("smart-open.json").to_str().unwrap().to_string();

        assert_eq!(get_path(), path);
    }
}