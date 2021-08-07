use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use which::which;
use std::process::Command;

fn find_python() -> which::Result<PathBuf> {
    let python_candidate = which("python")?;
    Ok(python_candidate)
}

fn install_python_dependencies(python_path: &Path) -> std::io::Result<std::process::Output> {
    Command::new(python_path)
        .args(&["-m", "pip", "install", "pylint"])
        .output()
}

fn find_vs_code() -> which::Result<PathBuf> {
    let vs_code_candidate = which("code")?;
    Ok(vs_code_candidate)
}

fn main() -> std::io::Result<()> {
    let mut log = File::create("smauglys.log")?;
    writeln!(log, "Installer started.").unwrap();
    let python_path = find_python().unwrap_or_else(|err| {
        writeln!(log, "Failed to locate Python: {}", err).unwrap();
        panic!("Failed to find Python");
    });
    writeln!(log, "Python path: {:?}", python_path).unwrap();
    let output = install_python_dependencies(&python_path).unwrap_or_else(|err| {
        writeln!(log, "Failed to start installation of Python dependencies: {}", err).unwrap();
        panic!("Failed to install Python dependencies");
    });
    writeln!(log, "Python dependencies installation stdout:").unwrap();
    log.write_all(&output.stdout).unwrap();
    log.write_all(&output.stderr).unwrap();
    let vs_code_path = find_vs_code().unwrap_or_else(|err| {
        writeln!(log, "Failed to locate VS Code: {}", err).unwrap();
        panic!("Failed to find VS Code");
    });
    writeln!(log, "VS Code path: {:?}", vs_code_path).unwrap();
    Ok(())
}