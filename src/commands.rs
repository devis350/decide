use clap::ArgMatches;
use cmd_lib::run_cmd;
use rand::Rng;

use crate::error::Error;
use crate::structs::{Project, Storage};

pub fn edit(matches: &ArgMatches, storage: &mut Storage) -> Result<(), Error> {
    let old_name = String::from(matches.value_of("name")?);
    storage.projects.iter_mut().for_each(|p| {
        if p.name == old_name {
            *p = dbg!(Project {
                name: matches
                    .value_of("rename")
                    .map_or_else(|| old_name.clone(), String::from),
                command: matches
                    .value_of("command")
                    .map(String::from)
                    .or_else(|| p.command.clone()),
            });
        }
    });
    storage.save()?;
    Ok(())
}

pub fn decide(matches: &ArgMatches, storage: &mut Storage) -> Result<(), Error> {
    if storage.projects.is_empty() {
        println!("you have nothing to work on :( / :)");
    } else {
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
    }
    Ok(())
}

pub fn new(matches: &ArgMatches, storage: &mut Storage) -> Result<(), Error> {
    let project_name = String::from(matches.value_of("NAME")?);
    if storage.projects.iter().any(|p| p.name == project_name) {
        println!("the project {} already exists", project_name);
    } else {
        storage.projects.push(Project {
            name: project_name.clone(),
            command: matches.value_of("start_command").map(String::from),
        });
        println!("the project {} was added", project_name);
        storage.save()?;
    }
    Ok(())
}

pub fn remove(matches: &ArgMatches, storage: &mut Storage) -> Result<(), Error> {
    let project_name = String::from(matches.value_of("NAME")?);
    let mut found = false;
    storage.projects.retain(|x| {
        let matches = project_name != x.name;
        found = found || !matches;
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
    if storage.projects.len() == 0 {
        println!("There are no projects");
    } else {
        storage.projects.iter().for_each(|p| println!("{}", p.name));
    }
}
