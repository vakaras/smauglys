use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::Command;
use which::which;

const WRAPPER_BIN: &'static [u8] = include_bytes!("../../wrapper.exe");
const PYTHON_PACKAGES: &'static [&'static str] = &["pylint", "mypy"];

#[derive(Default)]
struct ErrorHandler {
    event_log: Vec<Vec<u8>>,
}

macro_rules! log {
    ( $self:ident, $( $x:expr ),* ) => {
        $self.event_log.push(format!($( $x ),*).into());
    }
}

enum Error {
    WhichError(which::Error),
    IoError(std::io::Error),
    WinlogError(winlog::Error),
}

impl From<which::Error> for Error {
    fn from(error: which::Error) -> Self {
        Self::WhichError(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}

impl From<winlog::Error> for Error {
    fn from(error: winlog::Error) -> Self {
        Self::WinlogError(error)
    }
}

type IResult<T> = Result<T, Error>;

fn run_command(ehandler: &mut ErrorHandler, command: &Path, args: &[&str]) -> std::io::Result<()> {
    let mut final_command = format!("Running: {:?}", command);
    for arg in args {
        final_command.push(' ');
        final_command.push_str(arg);
    }
    ehandler.event_log.push(final_command.into());
    let output = Command::new(command).args(args).output()?;
    log!(ehandler, "stdout:");
    ehandler.event_log.push(output.stdout);
    log!(ehandler, "stderr:");
    ehandler.event_log.push(output.stderr);
    Ok(())
}

fn prepare_python(ehandler: &mut ErrorHandler) -> IResult<()> {
    log!(ehandler, "[enter] prepare_python");
    let python_path = find_python()?;
    log!(ehandler, "[enter] python_candidate={:?}", python_path);
    pip_upgrade(ehandler, &python_path)?;
    for package in PYTHON_PACKAGES {
        let mut retries = 3;
        loop {
            let result = pip_install(ehandler, &python_path, package);
            if let Ok(()) = result {
                break;
            }
            retries -= 1;
            if retries == 0 {
                result?;
            }
            log!(ehandler, "retrying {} of 3…", 3 - retries);
        }
    }
    log!(ehandler, "[exit] prepare_python");
    Ok(())
}

fn find_python() -> IResult<PathBuf> {
    let python_candidate = which("python")?;
    Ok(python_candidate)
}

fn pip_upgrade(ehandler: &mut ErrorHandler, python_path: &Path) -> std::io::Result<()> {
    run_command(
        ehandler,
        python_path,
        &["-m", "pip", "install", "--upgrade", "pip"],
    )
}

fn pip_install(
    ehandler: &mut ErrorHandler,
    python_path: &Path,
    package: &str,
) -> std::io::Result<()> {
    run_command(ehandler, python_path, &["-m", "pip", "install", package])
}

fn prepare_vs_code(ehandler: &mut ErrorHandler) -> IResult<()> {
    log!(ehandler, "[enter] prepare_vs_code");
    let mut vs_code_path = find_vs_code()?;
    log!(ehandler, "VS Code.bat candidate={:?}", vs_code_path);
    vs_code_path.pop();
    log!(ehandler, "VS Code bin directory={:?}", vs_code_path);
    vs_code_path.pop();
    log!(ehandler, "VS Code directory={:?}", vs_code_path);
    let vs_code_exe_path = vs_code_path.join("code.exe");
    log!(ehandler, "vs_code_exe_path={:?}", vs_code_exe_path);
    let vs_code_original_path = vs_code_path.join("code_original.exe");
    log!(
        ehandler,
        "vs_code_original_path={:?}",
        vs_code_original_path
    );
    log!(
        ehandler,
        "renaming: {:?} -> {:?}",
        &vs_code_exe_path,
        &vs_code_original_path
    );
    std::fs::rename(&vs_code_exe_path, &vs_code_original_path)?;
    log!(ehandler, "creating wrapper at {:?}", &vs_code_exe_path);
    let mut wrapper = File::create(vs_code_exe_path)?;
    wrapper.write_all(WRAPPER_BIN)?;
    log!(ehandler, "[exit] prepare_vs_code");
    Ok(())
}

fn find_vs_code() -> which::Result<PathBuf> {
    let vs_code_candidate = which("code")?;
    Ok(vs_code_candidate)
}

fn install(ehandler: &mut ErrorHandler) -> IResult<()> {
    winlog::try_register("Smauglys")?;
    prepare_python(ehandler)?;
    prepare_vs_code(ehandler)?;
    Ok(())
}

fn main() {
    let mut ehandler = ErrorHandler::default();
    if let Err(error) = install(&mut ehandler) {
        let mut log = File::create("C:\\smauglys.log").unwrap();
        for event in ehandler.event_log {
            log.write_all(&event).unwrap();
            log.write_all(b"\n").unwrap();
        }
        match error {
            Error::WhichError(error) => {
                write!(log, "Error: {}", error).unwrap();
            }
            Error::IoError(error) => {
                write!(log, "Error: {}", error).unwrap();
            }
            Error::WinlogError(error) => {
                write!(log, "Error: {}", error).unwrap();
            }
        }
    }
}
