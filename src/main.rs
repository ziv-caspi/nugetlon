mod filesystem;
mod git;
mod runner;
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
    let result = filesystem::find_files_by_extentions(current, OsString::from("csproj"), true);
    if let Err(err) = result {
        println!("ERROR: {}", err);
        return;
    }
    let found_files = result.unwrap();

    println!("found files: {:?}", found_files);
    let total_result = runner::run_all(found_files, &args);
    println!("result for running on all projects: {:#?}", total_result);
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
