use std::{path::Path, process::Command};

use log::{debug, trace};

use crate::error::{Error, IResult};

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