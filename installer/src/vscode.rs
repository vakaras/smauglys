use log::{debug, trace};
use std::path::{Path, PathBuf};

use crate::{
    command::{extract_file, extract_zip, run_command},
    error::IResult,
    VSCODE_EXTENSIONS, VSCODE_EXTENSIONS_ZIP,
};

pub(crate) fn ensure_vscode(installer: &Path, tmp_dir: &Path) -> IResult {
    trace!("[enter] ensure_vscode({:?})", installer);
    extract_file(crate::VSCODE_INSTALLER, installer)?;
    set_extensions_path()?;
    install_vscode(installer)?;
    let vscode_path = get_vscode_path()?;
    install_extensions(&vscode_path, tmp_dir)?;
    trace!("[exit] ensure_vscode");
    Ok(())
}

fn install_vscode(path: &Path) -> IResult {
    trace!("[enter] install_vscode({:?})", path);
    run_command(path, &["/SILENT", "/mergetasks=!runcode"])?;
    trace!("[exit] install_vscode");
    Ok(())
}

fn get_vscode_path() -> IResult<PathBuf> {
    trace!("[enter] get_vscode_path()");
    let mut vscode_exe_path = PathBuf::from(std::env::var("PROGRAMFILES")?);
    vscode_exe_path.push("Microsoft VS Code");
    debug!("VS Code directory={:?}", vscode_exe_path);
    vscode_exe_path.push("code.exe");
    trace!("[exit] get_vscode_path()={:?}", vscode_exe_path);
    Ok(vscode_exe_path)
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

fn install_extensions(vscode_path: &Path, tmp_dir: &Path) -> IResult {
    trace!("[enter] install_extensions()");
    let zip = tmp_dir.join("vscode_extensions.zip");
    extract_zip(VSCODE_EXTENSIONS_ZIP, &zip)?;
    let extracted_path = tmp_dir.join("vscode_extensions");
    for &extension in VSCODE_EXTENSIONS {
        let extension_path = extracted_path.join(format!("{}.vsix", extension));
        run_command(
            vscode_path,
            &["--install-extension", extension_path.to_str().unwrap()],
        )?;
    }
    trace!("[exit] install_extensions()");
    Ok(())
}
