use std::{fmt::Debug, path::Path};

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

pub(crate) struct ExtensionInstaller {
    /// An unique identifier of the extension.
    pub(super) identifier: &'static str,
    /// Optional function to invoke after the installation.
    ///
    /// The function may return a string explaining the error that occurred.
    pub(super) post_install: Option<fn(&Path) -> Result<(), String>>,
}

impl From<&Extension> for ExtensionInstaller {
    fn from(extension: &Extension) -> Self {
        Self {
            identifier: extension.identifier,
            post_install: extension.post_install,
        }
    }
}

impl ExtensionInstaller {
    pub(super) fn state_as_line(&self) -> String {
        format!("Papildinys {}: TODO", self.identifier)
    }
}