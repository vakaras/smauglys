use log::{error, trace};
use std::path::PathBuf;
use std::sync::mpsc::Sender;
use tempfile::TempDir;

use crate::{error::IResult, gui::Message};
use crate::{python, vscode, wrapper};

struct State {
    _extract_dir: TempDir,
    python_installer: PathBuf,
    vscode_installer: PathBuf,
    root_dir: PathBuf,
}

impl Default for State {
    fn default() -> Self {
        let extract_dir = tempfile::tempdir().unwrap();
        Self {
            root_dir: extract_dir.path().to_owned(),
            python_installer: extract_dir.path().join("PythonInstaller.exe"),
            vscode_installer: extract_dir.path().join("VSCodeSetup.exe"),
            _extract_dir: extract_dir,
        }
    }
}

pub(crate) fn do_install(notice: nwg::NoticeSender, sender: Sender<Message>) -> IResult {
    let state = State::default();
    python::ensure_python(&state.python_installer)?;
    sender.send(Message::ProgressUpdate {
        progress: 1,
        details: "Python įdiegtas".to_string(),
    })?;
    notice.notice();
    vscode::ensure_vscode(&state.vscode_installer, &state.root_dir)?;
    sender.send(Message::ProgressUpdate {
        progress: 1,
        details: "VS Code įdiegtas".to_string(),
    })?;
    notice.notice();
    // wrapper::ensure_wrapper()?;
    sender.send(Message::Finished)?;
    notice.notice();
    Ok(())
}

pub(crate) fn install(notice: nwg::NoticeSender, sender: Sender<Message>) {
    trace!("[enter] install");
    match do_install(notice, sender.clone()) {
        Ok(()) => {}
        Err(error) => {
            error!("An error occurred while installing: {}", error);
            if let Err(send_error) = sender.send(Message::Abort(format!("Įvyko klaida: {}", error)))
            {
                error!(
                    "An error occurred while trying to report an error: {}",
                    send_error
                );
            }
            notice.notice();
        }
    }
    trace!("[exit] install");
}
