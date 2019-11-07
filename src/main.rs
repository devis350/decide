#![feature(try_trait)]

#[macro_use]
extern crate clap;
extern crate directories;
#[macro_use]
extern crate lazy_static;

use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::option::NoneError;
use std::path::PathBuf;

use clap::{App, ArgMatches};
use directories::BaseDirs;
use serde::de::Unexpected::Bytes;
use serde::{Deserialize, Serialize};
use toml::value;

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
    fn init() -> Self {
        let mut file = File::open(PathBuf::from(&*FILE_PATH)).unwrap();
        let mut string = String::new();
        file.read_to_string(&mut string);
        toml::from_str(string.as_ref()).unwrap()
    }

    fn save(&self) {
        let mut file = File::create(PathBuf::from(&*FILE_PATH)).unwrap();
        file.write_all(toml::to_string(self).unwrap().as_bytes());
    }
}
