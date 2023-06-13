use anyhow::{anyhow, ensure, Ok, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::CommandLineArgs;

pub fn try_update_project(project_path: &PathBuf, args: &CommandLineArgs) -> Result<bool> {
    if is_installed(&project_path, &args.package_name)? {
        update_project(&project_path, &args.package_name, &args.package_version)?;
        return Ok(true);
    }

    return Ok(false);
}

fn is_installed(project: &Path, package_name: &str) -> Result<bool> {
    let path = project.to_str().ok_or(anyhow!("path is not utf8"))?;
    let output = Command::new("dotnet")
        .args(["list", path, "package"])
        .output()?;
    ensure!(
        output.status.success(),
        "failure running `dotnet list package`"
    );

    let as_string = String::from_utf8(output.stdout)?;
    return Ok(as_string.contains(package_name));
}

fn update_project(project: &Path, package_name: &str, package_version: &str) -> Result<bool> {
    let path = project.to_str().ok_or(anyhow!("path is not utf8"))?;
    let output = Command::new("dotnet")
        .args(["add", path, "package", package_name, "-v", package_version])
        .output()?;

    Ok(output.status.success())
}
