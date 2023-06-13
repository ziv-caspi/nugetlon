mod git;
mod updater;
use anyhow::{ensure, Result};
use std::{
    env,
    ffi::OsString,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct CommandLineArgs {
    path: String,
    package_name: String,
    package_version: String,
    created_branch_name: String,
}

fn main() {
    let result = get_command_line_args();
    if let Err(err) = result {
        println!(
            "usage: nugetlon [PATH] [PACKAGE_NAME] [PACKAGE_VERSION] [BRANCH_NAME].\nERROR: {}",
            err
        );
        return;
    }
    let args = result.unwrap();

    println!("welcome to Nugetlon. your args: {:?}", args);
    let current = Path::new(&args.path).to_owned();
    println!("running on: {:?}", current);
    let result = find_files_by_extentions(current, OsString::from("csproj"), true);
    if let Err(err) = result {
        println!("ERROR: {}", err);
        return;
    }
    let found_files = result.unwrap();

    println!("found files: {:?}", found_files);
    let total_result = run_all(found_files, &args);
    println!("result for running on all projects: {:#?}", total_result);
}

fn run_all(paths: Vec<PathBuf>, args: &CommandLineArgs) -> Result<Vec<PathBuf>> {
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

fn find_files_by_extentions<'a>(
    dir: PathBuf,
    ext: OsString,
    recursive: bool,
) -> Result<Vec<PathBuf>> {
    let mut found_files: Vec<PathBuf> = vec![];

    for enrtry in std::fs::read_dir(dir)? {
        let entry = enrtry?;
        let path = entry.path();
        if path.is_dir() && recursive {
            let mut found = find_files_by_extentions(path, ext.to_owned(), recursive)?;
            found_files.append(&mut found);
        } else {
            if let Some(file_extention) = path.extension() {
                if file_extention == ext {
                    found_files.push(path);
                }
            }
        }
    }

    Ok(found_files)
}

fn get_command_line_args() -> Result<CommandLineArgs> {
    let args: Vec<String> = env::args().collect();
    ensure!(args.len() == 5, "user must specify all required params.");
    let path = (&args[1]).to_string();
    let package_name = (&args[2]).to_string();
    let package_version = (&args[3]).to_string();
    let created_branch_name = (&args[4]).to_string();

    Ok(CommandLineArgs {
        path,
        created_branch_name,
        package_name,
        package_version,
    })
}
