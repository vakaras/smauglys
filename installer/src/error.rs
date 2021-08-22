use std::{env::VarError, fmt::Display, path::PathBuf, sync::mpsc::SendError};

use crate::gui::Message;

#[derive(Debug)]
pub(crate) enum Error {
    IoError(std::io::Error),
    CommandFailed {
        command: PathBuf,
        stdout: String,
        stderr: String,
    },
    NwgError(nwg::NwgError),
    SendError(SendError<Message>),
    WhichError(which::Error),
    VarError(VarError),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}

impl From<nwg::NwgError> for Error {
    fn from(error: nwg::NwgError) -> Self {
        Self::NwgError(error)
    }
}

impl From<SendError<Message>> for Error {
    fn from(error: SendError<Message>) -> Self {
        Self::SendError(error)
    }
}

impl From<which::Error> for Error {
    fn from(error: which::Error) -> Self {
        Self::WhichError(error)
    }
}

impl From<VarError> for Error {
    fn from(error: VarError) -> Self {
        Self::VarError(error)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IoError(error) => Display::fmt(error, f),
            Error::CommandFailed { command, ..} => {
                write!(f, "Nepavyko įvykdyti komandos: {:?}", command)
            },
            Error::NwgError(error) => Display::fmt(error, f),
            Error::SendError(error) => Display::fmt(error, f),
            Error::WhichError(error) => Display::fmt(error, f),
            Error::VarError(error) => Display::fmt(error, f),
        }
    }
}

pub(crate) type IResult<T=()> = Result<T, Error>;