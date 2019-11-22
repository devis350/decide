#![feature(try_trait)]

#[macro_use]
extern crate clap;
extern crate directories;
#[macro_use]
extern crate lazy_static;


use clap::App;

use crate::commands::{decide, list, new, remove};
use crate::structs::Storage;

mod commands;
mod structs;

fn main() {
    let mut storage = Storage::init();
    let yml = load_yaml!("./../cli.yaml");
    let matches = App::from_yaml(yml).get_matches();
    match matches.subcommand() {
        ("new", Some(matches)) => new(matches, &mut storage),
        ("list", Some(_)) => list(&mut storage),
        ("remove", Some(matches)) => remove(matches, &mut storage),
        _ => decide(&matches, &mut storage),
    }
}
