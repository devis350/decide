use crate::structs::{Project, Storage};
use clap::ArgMatches;
use rand::Rng;

pub fn decide(storage: &mut Storage) {
    if storage.projects.len() > 0 {
        let index = rand::thread_rng().gen_range(0, storage.projects.len());
        println!("Work on {}", storage.projects.get(index).unwrap().name);
    } else {
        println!("you have nothing to work on :( / :)");
    }
}

pub fn new(matches: &ArgMatches, storage: &mut Storage) {
    let project_name = String::from(matches.value_of("NAME").unwrap());
    if !storage.projects.iter().any(|p| p.name == project_name) {
        storage.projects.push(Project {
            name: project_name,
            command: None,
        });
        storage.save()
    }
}

pub fn remove(matches: &ArgMatches, storage: &mut Storage) {
    let project_name = String::from(matches.value_of("NAME").unwrap());
    storage.projects.retain(|x| project_name != x.name);
    storage.save();
}

pub fn list(storage: &mut Storage) {
    storage.projects.iter().for_each(|p| println!("{}", p.name));
}
