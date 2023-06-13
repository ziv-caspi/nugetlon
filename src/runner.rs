use crate::{git, updater, CommandLineArgs};
use anyhow::Result;
use std::path::PathBuf;

pub fn run_all(paths: Vec<PathBuf>, args: &CommandLineArgs) -> Result<Vec<PathBuf>> {
    let mut touched_projects = vec![];

    for path in paths {
        match run_for_project(&path, &args) {
            Ok(sucess) => {
                println!("result for {:#?} is: {:#?}", &path, sucess);
                touched_projects.push(path);
            }
            Err(err) => println!("error while udpating project: {}", err),
        };
    }

    Ok(touched_projects)
}

fn run_for_project(path: &PathBuf, args: &CommandLineArgs) -> Result<bool> {
    let updated = updater::try_update_project(&path, &args)?;
    if !updated {
        return Ok(false);
    }

    println!("updated {:#?}, making git changes", &path);
    let success = git::commit_push(
        &path,
        &args.package_name,
        &args.package_version,
        &args.created_branch_name,
    )?;

    println!(
        "result for {:#?}. udpated: {:#?}, git: {:#?}",
        &path, updated, success
    );
    Ok(success)
}
