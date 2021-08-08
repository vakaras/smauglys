use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::Command;

const EXTENSIONS: &'static [&'static str] = &["ms-python.python"];

enum Error {
    IoError(std::io::Error),
    GlobPatternError(glob::PatternError),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}

impl From<glob::PatternError> for Error {
    fn from(error: glob::PatternError) -> Self {
        Self::GlobPatternError(error)
    }
}

type TResult<T> = Result<T, Error>;

fn get_vscode_original_exe() -> PathBuf {
    let current_exe = std::env::current_exe().expect("Failed to detect current exe.");
    let mut code_original_exe = current_exe;
    code_original_exe.set_file_name("code_original.exe");
    code_original_exe
}

fn install_extension(vscode_exe: &Path, extension: &str) -> std::io::Result<()> {
    let _ = Command::new(vscode_exe)
        .args(&["--install-extension", extension])
        .status()?;
    Ok(())
}

fn do_ensure_extensions(vscode_exe: &Path) -> TResult<()> {
    let home_dir = dirs::home_dir().unwrap();
    let mut vs_code_extensions_dir = home_dir;
    vs_code_extensions_dir.push(".vscode");
    vs_code_extensions_dir.push("extensions");
    let extensions_pattern = vs_code_extensions_dir.join("*");
    let mut installed_extensions = HashSet::new();
    if let Some(pattern_as_str) = extensions_pattern.to_str() {
        if let Ok(paths) = glob::glob(pattern_as_str) {
            for entry in paths {
                match entry {
                    Ok(path) => {
                        if let Some(file_name) = path.file_name() {
                            if let Some(file_name) = file_name.to_str() {
                                let parts: Vec<_> = file_name.splitn(3, "-").collect();
                                installed_extensions.insert(format!("{}-{}", parts[0], parts[1]));
                            }
                        }
                    }
                    Err(e) => eprintln!("{:?}", e),
                }
            }
        }
    }
    for extension in EXTENSIONS {
        if !installed_extensions.contains(*extension) {
            if let Err(error) = install_extension(vscode_exe, extension) {
                eprintln!("Error installing extension {}: {}", extension, error);
            }
        }
    }
    Ok(())
}

fn ensure_extensions(vscode_exe: &Path) {
    if let Err(error) = do_ensure_extensions(vscode_exe) {
        match error {
            Error::GlobPatternError(error) => eprintln!("Error: {}", error),
            Error::IoError(error) => eprintln!("Error: {}", error),
        }
    }
}

fn start_vs_code(vscode_exe: &Path) -> std::io::Result<()> {
    let mut args = std::env::args();
    args.next();
    let _ = Command::new(vscode_exe).args(args).status()?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let vscode_exe = get_vscode_original_exe();
    ensure_extensions(&vscode_exe);
    start_vs_code(&vscode_exe)
}
