use std::path::Path;
use log::trace;

use crate::{command::{extract_file, run_command}, error::IResult};

pub(crate) fn ensure_vscode(installer: &Path) -> IResult {
    trace!("[enter] ensure_vscode({:?})", installer);
    extract_file(crate::VSCODE_INSTALLER, installer)?;
    install_vscode(installer)?;
    trace!("[exit] ensure_vscode");
    Ok(())
}

fn install_vscode(path: &Path) -> IResult {
    trace!("[enter] install_vscode({:?})", path);
    run_command(path, &["/SILENT", "/mergetasks=!runcode"])?;
    trace!("[exit] install_vscode");
    Ok(())
}