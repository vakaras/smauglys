use std::{collections::HashSet, path::Path};
use log::{debug, trace};

mod descriptions;
mod extension;
mod gui;

pub(crate) use descriptions::*;
pub(crate) use extension::Extension;

enum QuickCheckError {
    NoHome,
}

/// Quickly check whether all extensions are installed.
fn quick_check(extensions: &[Extension]) -> Result<bool, QuickCheckError> {
    trace!("[enter] quick_check");
    let home_dir = dirs::home_dir().ok_or(QuickCheckError::NoHome)?;
    debug!("home_dir = {:?}", home_dir);
    let mut vs_code_extensions_dir = home_dir;
    vs_code_extensions_dir.push(".vscode");
    vs_code_extensions_dir.push("extensions");
    debug!(
        "vs_code_extensions_dir = {:?}",
        vs_code_extensions_dir
    );
    let extensions_pattern = vs_code_extensions_dir.join("*");
    debug!("extensions_pattern = {:?}", extensions_pattern);
    let mut installed_extensions = HashSet::new();
    if let Some(pattern_as_str) = extensions_pattern.to_str() {
        debug!("pattern_as_str = {:?}", pattern_as_str);
        if let Ok(paths) = glob::glob(pattern_as_str) {
            for entry in paths {
                match entry {
                    Ok(path) => {
                        debug!("entry.path = {:?}", path);
                        if let Some(file_name) = path.file_name() {
                            debug!("  file_name = {:?}", file_name);
                            if let Some(file_name) = file_name.to_str() {
                                let parts: Vec<_> = file_name.splitn(3, "-").collect();
                                if let (Some(publisher), Some(name)) = (parts.get(0), parts.get(1))
                                {
                                    let extension = format!("{}-{}", publisher, name);
                                    debug!("  found extension: {:?}", extension);
                                    installed_extensions.insert(extension);
                                }
                            }
                        }
                    }
                    Err(error) => debug!("error for glob entry: {}", error),
                }
            }
        }
    }
    for extension in extensions {
        debug!("checking extension: {:?}", extension);
        if installed_extensions.contains(extension.identifier) {
            debug!("  already installed");
        } else {
            debug!("  not installed");
            return Ok(false);
        }
    }
    Ok(true)
}

pub(crate) fn ensure_installed(vscode_exe: &Path, extensions: &[Extension]) {
    trace!("[enter] ensure_installed");
    if !quick_check(extensions).unwrap_or(false) {
        debug!("Quick check: not all extensions installed");
        // Not all extensions installed. Launch GUI.
        self::gui::run(vscode_exe, extensions);
    } else {
        debug!("Quick check: all extensions installed");
    }
    trace!("[exit] ensure_installed");
}