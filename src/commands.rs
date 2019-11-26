use clap::ArgMatches;
use rand::Rng;

use crate::error::Error;
use crate::structs::{Project, Storage};
use cmd_lib::run_cmd;

pub fn decide(matches: &ArgMatches, storage: &mut Storage) -> Result<(), Error> {
    if storage.projects.len() > 0 {
        let index = rand::thread_rng().gen_range(0, storage.projects.len());
        let project = storage.projects.get(index)?;
        if matches.is_present("start") {
            match &project.command {
                Some(cmd) => {
                    run_cmd(cmd)?;
                }
                None => println!(
                    "{} has no associated start command, non the less you should work on it",
                    project.name
                ),
            }
        } else {
            println!("Work on {}", project.name);
        }
    } else {
        println!("you have nothing to work on :( / :)");
    }
    Ok(())
}

pub fn new(matches: &ArgMatches, storage: &mut Storage) -> Result<(), Error> {
    let project_name = String::from(matches.value_of("NAME")?);
    if !storage.projects.iter().any(|p| p.name == project_name) {
        storage.projects.push(Project {
            name: project_name.clone(),
            command: matches.value_of("start_command").map(String::from),
        });
        println!("the project {} was added", project_name);
        storage.save()?;
    } else {
        println!("the project {} already exists", project_name);
    }
    Ok(())
}

pub fn remove(matches: &ArgMatches, storage: &mut Storage) -> Result<(), Error> {
    let project_name = String::from(matches.value_of("NAME")?);
    let mut found = false;
    storage.projects.retain(|x| {
        let matches = project_name != x.name;
        found = found || matches;
        matches
    });
    if found {
        println!("{} was removed", project_name)
    } else {
        println!("{} wasn't found", project_name)
    }
    storage.save()?;
    Ok(())
}

pub fn list(storage: &mut Storage) {
    storage.projects.iter().for_each(|p| println!("{}", p.name));
}
