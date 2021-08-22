use log::{debug, trace};
use std::path::Path;

mod descriptions;
mod extension;
mod gui;

pub(crate) use descriptions::*;
pub(crate) use extension::Extension;

use crate::code::GetExtError;

/// Quickly check whether all extensions are installed.
fn quick_check(extensions: &[Extension]) -> Result<bool, GetExtError> {
    trace!("[enter] quick_check");
    let installed_extensions = crate::code::get_installed_extensions_quick()?;
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
