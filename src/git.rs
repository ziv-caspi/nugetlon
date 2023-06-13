use anyhow::{anyhow, ensure, Result};
use std::{io::Write, path::PathBuf, process::Command};

pub fn commit_push(
    path: &PathBuf,
    package_name: &str,
    package_version: &str,
    branch_name: &str,
) -> Result<bool> {
    gen_script(path, branch_name, package_name, package_version)?;
    let out = Command::new("./git-script.bat").output()?;
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

pub fn gen_script(
    path: &PathBuf,
    branch_name: &str,
    package_name: &str,
    package_version: &str,
) -> Result<()> {
    let mut anncesstors = path.ancestors();
    anncesstors.next();
    let parent = anncesstors
        .next()
        .ok_or(anyhow!("could not get parent dir for file path"))?
        .to_str()
        .ok_or(anyhow!("path is not valid unicde"))?;

    let commit_msg = format!(
        "updated {} nuget to version {}",
        package_name, package_version
    );
    let script = format!(
        "cd {} \n git checkout dev \n :: git pull origin dev \n git branch --force {} \n git checkout {} \n git add . \n git commit -m \"{}\" \n :: git push",
        parent, branch_name, branch_name, commit_msg
    );

    let mut file = std::fs::File::create("./git-script.bat")?;
    file.write_all(script.as_bytes())?;

    Ok(())
}
