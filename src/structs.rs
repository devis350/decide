use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use directories::BaseDirs;
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref FILE_PATH: PathBuf = BaseDirs::new()
        .unwrap()
        .data_dir()
        .join("decision-cli.toml");
}

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub command: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Storage {
    pub projects: Vec<Project>,
}

impl Storage {
    fn new() -> Self {
        Self { projects: vec![] }
    }

    pub fn init() -> Self {
        match File::open(PathBuf::from(&*FILE_PATH)) {
            Ok(mut file) => {
                let mut string = String::new();
                file.read_to_string(&mut string)
                    .expect("couldn't read File to String");
                toml::from_str(string.as_ref()).expect("config file structure is incorrect")
            }
            Err(_) => Self::new(),
        }
    }

    pub fn save(&self) {
        let mut file = File::create(PathBuf::from(&*FILE_PATH)).unwrap();
        file.write_all(dbg!(toml::to_string(self).unwrap()).as_bytes())
            .expect("couldn't write String to File");
    }
}
