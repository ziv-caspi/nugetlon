use anyhow::{anyhow, Result};
use std::{env::current_exe, io::Write, path::PathBuf, process::Command};

pub fn pre_update(path: &PathBuf, base_branch: &str) -> Result<bool> {
    let parent = get_parent_path(path)?;
    let out = Command::new(prefix_path("pre-git-script.bat")?)
        .env("FOUND_DIR", parent)
        .env("SRC_BRANCH", base_branch)
        .output()?;
    Ok(out.status.success())
}

pub fn post_update(
    path: &PathBuf,
    package_name: &str,
    package_version: &str,
    branch_name: &str,
) -> Result<bool> {
    let parent = get_parent_path(path)?;
    let commit_msg = format!(
        "updated {} nuget to version {}",
        package_name, package_version
    );

    let out = Command::new(prefix_path("post-git-script.bat")?)
        .env("FOUND_DIR", parent)
        .env("BRANCH_NAME", branch_name)
        .env("COMMIT_MSG", commit_msg)
        .output()?;
    match out.status.success() {
        true => {
            println!("created and pushed new branch with changes!");
            return Ok(true);
        }
        false => {
            println!("error running script: {}", String::from_utf8(out.stderr)?);
            return Ok(false);
        }
    }
}

fn get_parent_path(path: &PathBuf) -> Result<&str> {
    let mut anncesstors = path.ancestors();
    anncesstors.next();
    let parent = anncesstors
        .next()
        .ok_or(anyhow!("could not get parent dir for file path"))?
        .to_str()
        .ok_or(anyhow!("path is not valid unicde"))?;
    Ok(parent)
}

fn prefix_path(path: &str) -> Result<String> {
    let current_exe = std::env::current_exe()?;
    let parent = get_parent_path(&current_exe)?;
    let prefixed = format!("{}\\{}", parent, path);
    Ok(prefixed)
}
