use log::{debug, trace};
use std::path::{Path, PathBuf};

use crate::{
    command::{extract_file, run_command},
    error::IResult,
    PYTHON_PACKAGES_ZIP,
};

pub(crate) fn ensure_python(installer: &Path, tmp_dir: &Path) -> IResult {
    trace!("[enter] ensure_python({:?})", installer);
    extract_file(crate::PYTHON_INSTALLER, installer)?;
    install_python(installer)?;
    install_packages(tmp_dir)?;
    trace!("[exit] ensure_python");
    Ok(())
}

fn install_python(path: &Path) -> IResult {
    trace!("[enter] install_python({:?})", path);
    run_command(path, &["/passive", "InstallAllUsers=1", "PrependPath=1"])?;
    trace!("[exit] install_python");
    Ok(())
}

fn install_packages(tmp_dir: &Path) -> IResult<()> {
    trace!("[enter] install_packages");
    let python_path = find_python()?;
    debug!("python_candidate={:?}", python_path);
    let zip = tmp_dir.join("python_packages.zip");
    extract_zip(PYTHON_PACKAGES_ZIP, &zip)?;
    let extracted_path = tmp_dir.join("python_packages");
    let requirements = extracted_path
        .join("requirements.txt")
        .into_os_string()
        .into_string()
        .unwrap();
    let find_links = extracted_path.into_os_string().into_string().unwrap();
    // pip_upgrade(&python_path)?;
    // pip install --no-index --find-links /path/to/download/dir/ -r requirements.txt
    run_command(
        python_path,
        &[
            "-m",
            "pip",
            "install",
            "--no-index",
            "--find-links",
            &find_links,
            "-r",
            &requirements,
        ],
    )?;
    // for package in crate::PYTHON_PACKAGES {
    //     let mut retries = 3;
    //     loop {
    //         let result = pip_install(&python_path, package);
    //         if let Ok(()) = result {
    //             break;
    //         }
    //         retries -= 1;
    //         if retries == 0 {
    //             result?;
    //         }
    //         debug!("retrying {} of 3…", 3 - retries);
    //     }
    // }
    debug!("[exit] install_packages");
    Ok(())
}

fn find_python() -> IResult<PathBuf> {
    let mut python_path = PathBuf::from(std::env::var("PROGRAMFILES")?);
    // Python 3.8 is the latest version still supported on Windows 7.
    python_path.push("Python38");
    python_path.push("python.exe");
    Ok(python_path)
}

// fn pip_upgrade(python_path: &Path) -> IResult {
//     run_command(python_path, &["-m", "pip", "install", "--upgrade", "pip"])
// }

// fn pip_install(python_path: &Path, package: &str) -> IResult {
//     run_command(python_path, &["-m", "pip", "install", package])
// }
