use std::{
    fmt::{Debug, Display},
    path::{Path, PathBuf},
};

use log::{debug, error};

use crate::code::{self, InstallExtError};

#[derive(Clone)]
pub(crate) struct Extension {
    /// An unique identifier of the extension.
    pub(super) identifier: &'static str,
    /// Optional function to invoke after the installation.
    ///
    /// The function may return a string explaining the error that occurred.
    pub(super) post_install: Option<fn(&Path) -> Result<(), String>>,
}

impl Debug for Extension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Extension({})", self.identifier)
    }
}

#[derive(Debug, Clone)]
enum ExtensionState {
    Unknown,
    ToBeInstalled,
    Installed,
    InstallationError(String),
}

impl Display for ExtensionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExtensionState::Unknown => {
                write!(f, "tikrinama")
            }
            ExtensionState::ToBeInstalled => {
                write!(f, "diegiama")
            }
            ExtensionState::Installed => {
                write!(f, "sėkmingai įdiegta")
            }
            ExtensionState::InstallationError(message) => {
                write!(f, "diegimas nepavyko\r\n  {}", message)
            }
        }
    }
}

#[derive(Clone)]
struct ExtensionInstaller {
    /// An unique identifier of the extension.
    identifier: &'static str,
    /// Optional function to invoke after the installation.
    ///
    /// The function may return a string explaining the error that occurred.
    post_install: Option<fn(&Path) -> Result<(), String>>,
    state: ExtensionState,
}

impl From<&Extension> for ExtensionInstaller {
    fn from(extension: &Extension) -> Self {
        Self {
            identifier: extension.identifier,
            post_install: extension.post_install,
            state: ExtensionState::Unknown,
        }
    }
}

impl ExtensionInstaller {
    fn state_as_line(&self) -> String {
        format!("Papildinys {}: {}\r\n", self.identifier, self.state)
    }
    fn is_finished(&self) -> bool {
        match self.state {
            ExtensionState::Unknown | ExtensionState::ToBeInstalled => false,
            ExtensionState::Installed | ExtensionState::InstallationError(_) => true,
        }
    }
}

pub(crate) struct ExtensionInstallerList {
    vscode_exe: PathBuf,
    extensions: Vec<ExtensionInstaller>,
    abort_message: Option<String>,
    initialized: bool,
}

impl ExtensionInstallerList {
    pub(crate) fn new(vscode_exe: &Path, extensions: &[Extension]) -> Self {
        Self {
            vscode_exe: vscode_exe.to_owned(),
            extensions: extensions
                .iter()
                .map(|extension| extension.into())
                .collect(),
            abort_message: None,
            initialized: false,
        }
    }
    pub(crate) fn get_state(&self, buf: &mut String) {
        for extension in &self.extensions {
            buf.push_str(&extension.state_as_line());
        }
    }
    pub(crate) fn get_current_progress(&self) -> u32 {
        self.extensions
            .iter()
            .filter(|extension| extension.is_finished())
            .count() as u32
    }
    pub(crate) fn is_finished(&self) -> bool {
        self.extensions
            .iter()
            .all(|extension| extension.is_finished())
    }
    pub(crate) fn process_next_action(&mut self) {
        if !self.initialized {
            self.initialize();
        } else {
            self.install_next_extension();
        }
    }
    fn abort(&mut self, message: String) {
        self.abort_message = Some(message);
    }
    pub(crate) fn get_abort_message(&self) -> Option<String> {
        self.abort_message.clone()
    }
    fn initialize(&mut self) {
        let installed_extensions = match code::get_installed_extensions(&self.vscode_exe) {
            Ok(extensions) => extensions,
            Err(error) => {
                self.abort(format!("Nepavyko paleisti VS Code: {}", error));
                return;
            }
        };
        for extension in &mut self.extensions {
            debug!("extension state (should be Unknown): {:?}", extension.state);
            if installed_extensions.contains(extension.identifier) {
                extension.state = ExtensionState::Installed;
            } else {
                extension.state = ExtensionState::ToBeInstalled;
            }
        }
        self.initialized = true;
    }
    fn install_next_extension(&mut self) {
        let next_extension = self
            .extensions
            .iter_mut()
            .filter(|extension| !extension.is_finished())
            .next();
        if let Some(extension) = next_extension {
            match code::install_extension(&self.vscode_exe, extension.identifier) {
                Ok(()) => {
                    if let Some(post_install) = extension.post_install {
                        match post_install(&self.vscode_exe) {
                            Ok(()) => {
                                extension.state = ExtensionState::Installed;
                            }
                            Err(message) => {
                                error!("post_install({}) error: {}", extension.identifier, message);
                                extension.state = ExtensionState::InstallationError(message);
                            }
                        }
                    } else {
                        extension.state = ExtensionState::Installed;
                    }
                }
                Err(InstallExtError::IoError(error)) => {
                    error!(
                        "InstallExtError::IoError({}) error: {}",
                        extension.identifier, error
                    );
                    extension.state =
                        ExtensionState::InstallationError(format!("Klaida: {}", error));
                }
                Err(InstallExtError::ExecutionFailed { stdout, stderr }) => {
                    error!(
                        "InstallExtError::ExecutionFailed({}) \r\nstdout: {}\r\nstderr: {}",
                        extension.identifier, stdout, stderr
                    );
                    extension.state = ExtensionState::InstallationError(format!(
                        "VS Code derinimo informacija:\r\n{}\r\n{}\r\n",
                        stdout, stderr
                    ));
                }
            }
        }
    }
}
