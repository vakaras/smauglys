use std::path::PathBuf;
use std::{path::Path, process::Command, sync::mpsc::Sender};
use std::fs::File;
use std::io::prelude::*;
use log::{debug, trace, error};
use tempfile::TempDir;

use crate::{python, vscode, wrapper};
use crate::{error::{Error, IResult}, gui::Message};

struct State {
    _extract_dir: TempDir,
    python_installer: PathBuf,
    vscode_installer: PathBuf,
    wrapper_bin: PathBuf,
}

impl Default for State {
    fn default() -> Self {
        let extract_dir = tempfile::tempdir().unwrap();
        Self {
            python_installer: extract_dir.path().join("PythonInstaller.exe"),
            vscode_installer: extract_dir.path().join("VSCodeSetup.exe"),
            wrapper_bin: extract_dir.path().join("wrapper.exe"),
            _extract_dir: extract_dir,
        }
    }
}

fn extract_file(bytes: &[u8], path: &Path) -> std::io::Result<()> {
    trace!("[enter] extract_file({:?})", path);
    let mut file = File::create(path)?;
    file.write_all(bytes)?;
    trace!("[exit] extract_file");
    Ok(())
}

pub(crate) fn do_install(notice: nwg::NoticeSender, sender: Sender<Message>) -> IResult {
    let state = State::default();
    python::ensure_python(&state.python_installer)?;
    sender.send(Message::ProgressUpdate {
        progress: 1,
        details: "Python įdiegtas".to_string(),
    })?;
    notice.notice();
    vscode::ensure_vscode(&state.python_installer)?;
    sender.send(Message::ProgressUpdate {
        progress: 1,
        details: "VS Code įdiegtas".to_string(),
    })?;
    notice.notice();
    wrapper::ensure_wrapper(&state.wrapper_bin)?;
    sender.send(Message::Finished)?;
    notice.notice();
    // extract_file(PYTHON_INSTALLER, &state.python_installer).unwrap();
    // extract_file(VSCODE_INSTALLER, &state.vscode_installer).unwrap();
    // extract_file(WRAPPER_BIN, &state.wrapper_bin).unwrap();
    // install_python(&state.python_installer).unwrap();
    // install_vscode(&state.vscode_installer).unwrap();
    Ok(())
}

pub(crate) fn install(notice: nwg::NoticeSender, sender: Sender<Message>) {
    trace!("[enter] install");
    match do_install(notice, sender.clone()) {
        Ok(()) => {},
        Err(error) => {
            error!("An error occurred while installing: {}", error);
            if let Err(send_error) = sender.send(Message::Abort(format!("Įvyko klaida: {}", error))) {
                error!("An error occurred while trying to report an error: {}", send_error);
            }
            notice.notice();
        }
    }
    trace!("[exit] install");
}