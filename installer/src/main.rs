// #![windows_subsystem = "windows"]

extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

const PYTHON_INSTALLER: &'static [u8] = include_bytes!("../../PythonInstaller.exe");
const VSCODE_INSTALLER: &'static [u8] = include_bytes!("../../VSCodeSetup.exe");
const WRAPPER_BIN: &'static [u8] = include_bytes!("../../wrapper.exe");
const PYTHON_PACKAGES: &'static [&'static str] = &["pylint", "mypy"];

mod command;
mod error;
mod gui;
mod installation;
mod python;
mod vscode;
mod wrapper;

use log::{debug, error};

fn main() {
    winlog::register("Smauglys");
    winlog::init("Smauglys").unwrap();
    debug!("Starting Smauglys installer");
    match gui::run() {
        Ok(()) => {}
        Err(error) => {
            error!("An error occurred: {}", error);
        }
    }
}
