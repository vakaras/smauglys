use log::{debug, trace};
use std::fs::File;
use std::io::prelude::*;
use std::{path::Path, process::Command};

use crate::error::{Error, IResult};

pub(crate) fn run_command(command: &Path, args: &[&str]) -> IResult {
    let mut final_command = format!("Running: {:?}", command);
    for arg in args {
        final_command.push(' ');
        final_command.push_str(arg);
    }
    let output = Command::new(command).args(args).output()?;
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
            command: command.to_path_buf(),
            stdout: stdout.to_string(),
            stderr: stderr.to_string(),
        })
    }
}

pub(crate) fn extract_file(bytes: &[u8], path: &Path) -> std::io::Result<()> {
    trace!("[enter] extract_file({:?})", path);
    let mut file = File::create(path)?;
    file.write_all(bytes)?;
    trace!("[exit] extract_file");
    Ok(())
}
