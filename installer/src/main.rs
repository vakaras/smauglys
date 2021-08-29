#![windows_subsystem = "windows"]

extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

const PYTHON_INSTALLER: &'static [u8] = include_bytes!("../../PythonInstaller.exe");
const VSCODE_INSTALLER: &'static [u8] = include_bytes!("../../VSCodeSetup.exe");
const WRAPPER_BIN: &'static [u8] = include_bytes!("../../wrapper.exe");
const VSCODE_EXTENSIONS: &'static [&'static str] = &[
    "vakaras.vscode-language-pack-lt",
    "ms-python.vscode-pylance",
    "ms-toolsai.jupyter",
    "ms-python.python",
    "hediet.debug-visualizer",
];
const VSCODE_EXTENSIONS_ZIP: &'static [u8] = include_bytes!("../../vscode_extensions.zip");
const PYTHON_PACKAGES_ZIP: &'static [u8] = include_bytes!("../../python_packages.zip");
const PYTHON_REQUIREMENTS: &'static [u8] = include_bytes!("../../python-requirements.txt");
const PYTHON_PACKAGES: &'static [&'static str] = &["pylint", "mypy"];
const PYTHON_LICENSE: &'static str = include_str!("../../PYTHON-LICENSE-3.8.10.txt");
const VSCODE_LICENSE: &'static str = include_str!("../../VSCODE-LICENSE.rtf");

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
