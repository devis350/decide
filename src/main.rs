#![feature(try_trait)]

#[macro_use]
extern crate clap;
extern crate directories;
#[macro_use]
extern crate lazy_static;

use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use clap::{App, ArgMatches};
use directories::BaseDirs;
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref FILE_PATH: PathBuf = BaseDirs::new()
        .unwrap()
        .data_dir()
        .join("decision-cli.toml");
}

fn main() {
    let yaml = load_yaml!("./../cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let mut storage = Storage::init();
    match matches.subcommand() {
        ("new", Some(matches)) => new(matches, &mut storage),
        _ => {}
    }
    storage.save();
}

fn new(matches: &ArgMatches, storage: &mut Storage) {
    let project = String::from(matches.value_of("NAME").unwrap());
    if !storage.projects.contains(&project) {
        storage.projects.push(project)
    }
}

#[derive(Serialize, Deserialize)]
struct Storage {
    projects: Vec<String>,
    ideas: Vec<String>,
}

impl Storage {
    fn new() -> Self {
        Self {
            projects: vec![],
            ideas: vec![],
        }
    }

    fn init() -> Self {
        match File::open(PathBuf::from(&*FILE_PATH)) {
            Ok(mut file) => {
                let mut string = String::new();
                file.read_to_string(&mut string);
                toml::from_str(string.as_ref()).unwrap()
            }
            Err(_) => Self::new(),
        }
    }

    fn save(&self) {
        let mut file = File::create(PathBuf::from(&*FILE_PATH)).unwrap();
        file.write_all(toml::to_string(self).unwrap().as_bytes());
    }
}
