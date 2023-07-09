use anyhow::{anyhow, Result};
use std::path::Path;
use xmltree::EmitterConfig;

pub fn xml_tree_update_project(
    project: &Path,
    package_pattern: &str,
    package_version: &str,
) -> Result<i32> {
    let contents = std::fs::read_to_string(project)?;
    let mut document = xmltree::Element::parse(contents.as_bytes())?;
    let mut modified = 0;

    for child in &mut document.children {
        if let Some(item_group) = child.as_mut_element() {
            if item_group.name != "ItemGroup" {
                continue;
            }

            for item in &mut item_group.children {
                if let Some(package_reference) = item.as_mut_element() {
                    let name = package_reference
                        .attributes
                        .get("Include")
                        .ok_or(anyhow!("package does not have include attribute"))?;

                    if !(name
                        .to_lowercase()
                        .contains(&package_pattern.to_lowercase()))
                    {
                        continue;
                    }

                    let version = package_reference
                        .attributes
                        .get_mut("Version")
                        .ok_or(anyhow!("package does not have version attribute"))?;
                    *version = package_version.to_owned();
                    modified += 1;
                }
            }
        }
    }

    if modified == 0 {
        return Ok(modified);
    }

    let file = std::fs::File::create(project)?;
    document.write_with_config(file, EmitterConfig::new().perform_indent(true))?;
    Ok(modified)
}
