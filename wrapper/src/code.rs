use std::{path::{Path, PathBuf}, process::{Command}};

/// Get the path to the original VS Code executable.
pub(crate) fn get_vscode_original_exe() -> PathBuf {
    let current_exe = std::env::current_exe().expect("Failed to detect current exe.");
    let mut code_original_exe = current_exe;
    code_original_exe.set_file_name("code_original.exe");
    code_original_exe
}

/// Start VS Code with the same arguments as this executable.
pub(crate) fn start_vs_code(vscode_exe: &Path) -> std::io::Result<()> {
    let mut args = std::env::args();
    let _command = args.next();
    let exit_status = Command::new(vscode_exe).args(args).status()?;
    assert!(exit_status.success(), "VS Code exited with an error: {}", exit_status);
    Ok(())
}