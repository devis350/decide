use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use directories::BaseDirs;
use serde::{Deserialize, Serialize};

use crate::error::Error;

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

    pub fn init() -> Result<Self, Error> {
        Ok(match File::open(PathBuf::from(&*FILE_PATH)) {
            Ok(mut file) => {
                let mut string = String::new();
                file.read_to_string(&mut string)?;
                toml::from_str(string.as_ref())?
            }
            Err(_) => Self::new(),
        })
    }

    pub fn save(&self) -> Result<(), Error> {
        let mut file = File::create(PathBuf::from(&*FILE_PATH))?;
        file.write_all(toml::to_string(self)?.as_bytes())?;
        Ok(())
    }
}
