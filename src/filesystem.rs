use anyhow::Result;
use std::{ffi::OsString, path::PathBuf};

pub fn find_files_by_extentions<'a>(
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
