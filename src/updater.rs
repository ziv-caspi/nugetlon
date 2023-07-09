use crate::xml_updater::xml_tree_update_project;
use crate::CommandLineArgs;
use anyhow::{Ok, Result};
use std::path::PathBuf;

pub fn try_update_project(project_path: &PathBuf, args: &CommandLineArgs) -> Result<bool> {
    let updated =
        xml_tree_update_project(&project_path, &args.package_name, &args.package_version)?;
    return Ok(updated > 0);
}
