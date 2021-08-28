use log::trace;
use std::path::{Path, PathBuf};

use crate::{
    command::{extract_file, run_command},
    error::IResult,
};

pub(crate) fn ensure_vscode(installer: &Path) -> IResult {
    trace!("[enter] ensure_vscode({:?})", installer);
    extract_file(crate::VSCODE_INSTALLER, installer)?;
    set_extensions_path()?;
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

fn set_extensions_path() -> IResult {
    trace!("[enter] set_extensions_path");
    let mut extensions_path = PathBuf::from(std::env::var("PROGRAMFILES")?);
    extensions_path.push("VS Code Extensions");
    run_command(
        "setx",
        &[
            "/M",
            "VSCODE_EXTENSIONS",
            extensions_path
                .into_os_string()
                .into_string()
                .unwrap()
                .as_str(),
        ],
    )?;
    trace!("[exit] set_extensions_path");
    Ok(())
}
