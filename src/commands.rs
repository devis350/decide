use clap::ArgMatches;
use rand::Rng;

use crate::structs::{Project, Storage};
use std::process::Command;

pub fn decide(storage: &mut Storage) {
    if storage.projects.len() > 0 {
        let index = rand::thread_rng().gen_range(0, storage.projects.len());
        let project = storage.projects.get(index).unwrap();
        match &project.command {
            Some(command)=>{
                println!("toto:africa: open project with start_command");
            },
            None=> println!("Work on {}", project.name)
        };
    } else {
        println!("you have nothing to work on :( / :)");
    }
}

pub fn new(matches: &ArgMatches, storage: &mut Storage) {
    let project_name = String::from(matches.value_of("NAME").unwrap());
    if !storage.projects.iter().any(|p| p.name == project_name) {
        storage.projects.push(Project {
            name: project_name.clone(),
            command: None,
        });
        println!("the project {} was added", project_name);
        storage.save();
    } else {
        println!("the project {} already exists", project_name);
    }
}

pub fn remove(matches: &ArgMatches, storage: &mut Storage) {
    let project_name = String::from(matches.value_of("NAME").unwrap());
    let mut found = false;
    storage.projects.retain(|x| {
        let matches = project_name != x.name;
        found = found || matches;
        matches
    });
    if found { println!("{} was removed", project_name) } else { println!("{} wasn't found", project_name) }
    storage.save();
}

pub fn list(storage: &mut Storage) {
    storage.projects.iter().for_each(|p| println!("{}", p.name));
}
