use log::{debug, trace};
use std::{
    path::{Path, PathBuf},
    process::Command,
};

use crate::{
    command::{extract_file, extract_zip, run_command},
    error::{Error, IResult},
    VSCODE_EXTENSIONS, VSCODE_EXTENSIONS_ZIP,
};

pub(crate) fn ensure_vscode(installer: &Path, tmp_dir: &Path) -> IResult {
    trace!("[enter] ensure_vscode({:?})", installer);
    extract_file(crate::VSCODE_INSTALLER, installer)?;
    let extensions_path = set_extensions_path()?;
    install_vscode(installer)?;
    let vscode_path = get_vscode_path()?;
    install_extensions(&vscode_path, tmp_dir, &extensions_path)?;
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
    vscode_exe_path.push("VSCodium");
    debug!("VS Code directory={:?}", vscode_exe_path);
    vscode_exe_path.push("code.exe");
    trace!("[exit] get_vscode_path()={:?}", vscode_exe_path);
    Ok(vscode_exe_path)
}

fn set_extensions_path() -> IResult<String> {
    trace!("[enter] set_extensions_path");
    let mut extensions_path = PathBuf::from(std::env::var("PROGRAMFILES")?);
    extensions_path.push("VS Code Extensions");
    let extensions_path = extensions_path.into_os_string().into_string().unwrap();
    run_command("setx", &["/M", "VSCODE_EXTENSIONS", &extensions_path])?;
    trace!("[exit] set_extensions_path");
    Ok(extensions_path)
}

fn run_vscode_command(vscode_path: &Path, extensions_path: &str, args: &[&str]) -> IResult {
    trace!(
        "[enter] run_vscode_command(vscode_exe={:?}, extensions_path={:?}, args={:?})",
        vscode_path,
        extensions_path,
        args
    );
    let mut cli_path = vscode_path.to_path_buf();
    cli_path.pop();
    cli_path.push("resources");
    cli_path.push("app");
    cli_path.push("out");
    cli_path.push("cli.js");

    let mut final_command = format!("Running: {:?} {:?}", vscode_path, cli_path);
    for arg in args {
        final_command.push(' ');
        final_command.push_str(arg);
    }

    let output = Command::new(vscode_path)
        .env("ELECTRON_RUN_AS_NODE", "1")
        // This is needed because the setx command will take into affect only
        // after the application is restarted.
        .env("VSCODE_EXTENSIONS", extensions_path)
        .arg(cli_path)
        .args(args)
        .output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    debug!(
        "Running command: {}\r\nstdout: {}\r\nstderr: {}\r\n{:?}",
        final_command, stdout, stderr, output.status
    );
    if output.status.success() {
        Ok(())
    } else {
        Err(Error::CommandFailed {
            command: vscode_path.to_string_lossy().into_owned(),
            stdout: stdout.to_string(),
            stderr: stderr.to_string(),
        })
    }
}

fn install_extensions(vscode_path: &Path, tmp_dir: &Path, extensions_path: &str) -> IResult {
    trace!("[enter] install_extensions()");
    let zip = tmp_dir.join("vscode_extensions.zip");
    extract_zip(VSCODE_EXTENSIONS_ZIP, &zip)?;
    let extracted_path = tmp_dir.join("vscode_extensions");
    for &extension in VSCODE_EXTENSIONS {
        let extension_path = extracted_path.join(format!("{}.vsix", extension));
        run_vscode_command(
            vscode_path,
            extensions_path,
            &["--install-extension", extension_path.to_str().unwrap()],
        )?;
    }
    trace!("[exit] install_extensions()");
    Ok(())
}
