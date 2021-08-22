use std::{collections::HashSet, fmt::Display, path::{Path, PathBuf}, process::{Command}};

use log::{debug, trace};

/// Get the path to the original VS Code executable.
pub(crate) fn get_vscode_original_exe() -> PathBuf {
    let current_exe = std::env::current_exe().expect("Failed to detect current exe.");
    let mut code_original_exe = current_exe;
    code_original_exe.set_file_name("code_original.exe");
    code_original_exe
}

fn create_vs_code_command(vscode_exe: &Path, args: &[&str]) -> Command {
    trace!(
        "[enter] create_vs_code_command(vscode_exe={:?}, args={:?})",
        vscode_exe,
        args
    );
    let mut cli_path = vscode_exe.to_path_buf();
    cli_path.pop();
    cli_path.push("resources");
    cli_path.push("app");
    cli_path.push("out");
    cli_path.push("cli.js");

    let mut command = Command::new(vscode_exe);
    command
        .env("ELECTRON_RUN_AS_NODE", "1")
        .arg(cli_path)
        .args(args);
    trace!("[exit] create_vs_code_command");
    command
}

/// Start VS Code with the same arguments as this executable.
pub(crate) fn start_vs_code(vscode_exe: &Path) -> std::io::Result<()> {
    let mut args = std::env::args();
    let _command = args.next();
    let exit_status = Command::new(vscode_exe).args(args).status()?;
    assert!(exit_status.success(), "VS Code exited with an error: {}", exit_status);
    Ok(())
}

pub(crate) enum GetExtError {
    IoError(std::io::Error),
    FromUtf8Error(std::string::FromUtf8Error),
}

impl Display for GetExtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetExtError::IoError(error) => Display::fmt(error, f),
            GetExtError::FromUtf8Error(error) => Display::fmt(error, f),
        }
    }
}

impl From<std::io::Error> for GetExtError {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}

impl From<std::string::FromUtf8Error> for GetExtError {
    fn from(error: std::string::FromUtf8Error) -> Self {
        Self::FromUtf8Error(error)
    }
}

pub(crate) fn get_installed_extensions(vscode_exe: &Path) -> Result<HashSet<String>, GetExtError> {
    trace!("[enter] get_installed_extensions");
    let mut command = create_vs_code_command(vscode_exe, &["--list-extensions"]);
    let output = command.output()?;
    let stdout = String::from_utf8(output.stdout)?;
    debug!("stdout = {}", stdout);
    let extensions = stdout.trim().split_whitespace().map(ToOwned::to_owned).collect();
    trace!("[exit] get_installed_extensions");
    Ok(extensions)
}

pub(crate) enum InstallExtError {
    IoError(std::io::Error),
    ExecutionFailed {
        stdout: String, stderr: String,
    }
}

impl From<std::io::Error> for InstallExtError {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}

pub(crate) fn install_extension(vscode_exe: &Path, extension: &str) -> Result<(), InstallExtError> {
    trace!("[enter] install_extension({:?}, {:?})",
        vscode_exe,
        extension
    );
    let output =
        create_vs_code_command(vscode_exe, &["", "--install-extension", extension])
            .output()?;
    debug!("stdout raw: {:?}", &output.stdout);
    let stdout = String::from_utf8_lossy(&output.stdout);
    debug!("stdout:\n{:?}", stdout);
    debug!("stderr raw: {:?}", &output.stderr);
    let stderr = String::from_utf8_lossy(&output.stderr);
    debug!("stderr:\n{:?}", stderr);
    trace!("[exit] install_extension");
    if output.status.success() {
        Ok(())
    } else {
        Err(InstallExtError::ExecutionFailed {
            stdout: stdout.to_string(), stderr: stderr.to_string(),
        })
    }
}