use anyhow::{anyhow, ensure, Ok, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::xml::xml_tree_update_project;
use crate::CommandLineArgs;

pub fn try_update_project(project_path: &PathBuf, args: &CommandLineArgs) -> Result<bool> {
    let updated =
        xml_tree_update_project(&project_path, &args.package_name, &args.package_version)?;
    return Ok(updated > 0);
}
