#![feature(try_trait)]

#[macro_use]
extern crate clap;
extern crate directories;
#[macro_use]
extern crate lazy_static;

use crate::commands::{decide, list, new, remove};
use crate::structs::Storage;
use clap::App;

mod commands;
mod structs;

fn main() {
    let yaml = load_yaml!("./../cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let mut storage = Storage::init();
    match matches.subcommand() {
        ("new", Some(matches)) => new(matches, &mut storage),
        ("list", Some(_)) => list(&mut storage),
        ("remove", Some(matches)) => remove(matches, &mut storage),
        _ => decide(&mut storage),
    }
}
