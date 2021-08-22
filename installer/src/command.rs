use std::path::PathBuf;
use std::{path::Path, process::Command, sync::mpsc::Sender};
use std::fs::File;
use std::io::prelude::*;
use log::{debug, trace, error};
use tempfile::TempDir;

use crate::{error::{Error, IResult}, gui::Message};

fn run_command(command: &Path, args: &[&str]) -> IResult {
    let mut final_command = format!("Running: {:?}", command);
    for arg in args {
        final_command.push(' ');
        final_command.push_str(arg);
    }
    let output = Command::new(command).args(args).output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    debug!("Running command: {}\r\nstdout: {}\r\nstderr: {}\r\n{:?}", final_command, stdout, stderr, output.status);
    if output.status.success() {
        Ok(())
    } else {
        Err(Error::CommandFailed {
            command: command.to_path_buf(),
            stdout: stdout.to_string(),
            stderr: stderr.to_string(),
        })
    }
}

fn install_python(path: &Path) -> IResult {
    trace!("[enter] install_python({:?})", path);
    run_command(path, &["/passive", "InstallAllUsers=1", "PrependPath=1"])?;
    trace!("[exit] install_python");
    Ok(())
}

fn install_vscode(path: &Path) -> IResult {
    trace!("[enter] install_vscode({:?})", path);
    run_command(path, &["/SILENT", "/mergetasks=!runcode"])?;
    trace!("[exit] install_vscode");
    Ok(())
}

struct State {
    _extract_dir: TempDir,
    python_installer: PathBuf,
    vscode_installer: PathBuf,
    wrapper_bin: PathBuf,
}

impl Default for State {
    fn default() -> Self {
        let extract_dir = tempfile::tempdir().unwrap();
        Self {
            python_installer: extract_dir.path().join("PythonInstaller.exe"),
            vscode_installer: extract_dir.path().join("VSCodeSetup.exe"),
            wrapper_bin: extract_dir.path().join("wrapper.exe"),
            _extract_dir: extract_dir,
        }
    }
}

fn extract_file(bytes: &[u8], path: &Path) -> std::io::Result<()> {
    trace!("[enter] extract_file({:?})", path);
    let mut file = File::create(path)?;
    file.write_all(bytes)?;
    trace!("[exit] extract_file");
    Ok(())
}

