use log::{debug, trace};
use std::path::{Path, PathBuf};

use crate::{
    command::{extract_file, run_command},
    error::IResult,
};

pub(crate) fn ensure_python(installer: &Path) -> IResult {
    trace!("[enter] ensure_python({:?})", installer);
    extract_file(crate::PYTHON_INSTALLER, installer)?;
    install_python(installer)?;
    install_packages()?;
    trace!("[exit] ensure_python");
    Ok(())
}

fn install_python(path: &Path) -> IResult {
    trace!("[enter] install_python({:?})", path);
    run_command(path, &["/passive", "InstallAllUsers=1", "PrependPath=1"])?;
    trace!("[exit] install_python");
    Ok(())
}

fn install_packages() -> IResult<()> {
    trace!("[enter] prepare_python");
    let python_path = find_python()?;
    debug!("python_candidate={:?}", python_path);
    pip_upgrade(&python_path)?;
    for package in crate::PYTHON_PACKAGES {
        let mut retries = 3;
        loop {
            let result = pip_install(&python_path, package);
            if let Ok(()) = result {
                break;
            }
            retries -= 1;
            if retries == 0 {
                result?;
            }
            debug!("retrying {} of 3…", 3 - retries);
        }
    }
    debug!("[exit] prepare_python");
    Ok(())
}

fn find_python() -> IResult<PathBuf> {
    let mut python_path = PathBuf::from(std::env::var("PROGRAMFILES")?);
    python_path.push("Python39");
    python_path.push("python.exe");
    Ok(python_path)
}

fn pip_upgrade(python_path: &Path) -> IResult {
    run_command(python_path, &["-m", "pip", "install", "--upgrade", "pip"])
}

fn pip_install(python_path: &Path, package: &str) -> IResult {
    run_command(python_path, &["-m", "pip", "install", package])
}
