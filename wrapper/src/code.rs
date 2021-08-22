use std::{collections::HashSet, fmt::Display, fs::File, path::{Path, PathBuf}, process::{Command}};

use log::{debug, trace};

/// Get the path to the original VS Code executable.
pub(crate) fn get_vscode_original_exe() -> PathBuf {
    let current_exe = std::env::current_exe().expect("Failed to detect current exe.");
    let mut code_original_exe = current_exe;
    code_original_exe.set_file_name("code_original.exe");
    code_original_exe
}

fn create_vs_code_command(vscode_exe: &Path, args: &[&str]) -> Command {
    trace!(
        "[enter] create_vs_code_command(vscode_exe={:?}, args={:?})",
        vscode_exe,
        args
    );
    let mut cli_path = vscode_exe.to_path_buf();
    cli_path.pop();
    cli_path.push("resources");
    cli_path.push("app");
    cli_path.push("out");
    cli_path.push("cli.js");

    let mut command = Command::new(vscode_exe);
    command
        .env("ELECTRON_RUN_AS_NODE", "1")
        .arg(cli_path)
        .args(args);
    trace!("[exit] create_vs_code_command");
    command
}

/// Start VS Code with the same arguments as this executable.
pub(crate) fn start_vs_code(vscode_exe: &Path) -> std::io::Result<()> {
    let mut args = std::env::args();
    let _command = args.next();
    let exit_status = Command::new(vscode_exe).args(args).status()?;
    assert!(exit_status.success(), "VS Code exited with an error: {}", exit_status);
    Ok(())
}

pub(crate) enum GetExtError {
    NoHomeDir,
    NoConfigDir,
    InvalidJson(PathBuf),
    IoError(std::io::Error),
    FromUtf8Error(std::string::FromUtf8Error),
    JsonError(serde_json::Error),
}

impl Display for GetExtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GetExtError::NoHomeDir => write!(f, "nerastas namų katalogas"),
            GetExtError::NoConfigDir => write!(f, "nerastas nustatymų katalogas"),
            GetExtError::InvalidJson(path) => write!(f, "pažeistas JSON failas: {:?}", path),
            GetExtError::IoError(error) => Display::fmt(error, f),
            GetExtError::FromUtf8Error(error) => Display::fmt(error, f),
            GetExtError::JsonError(error) => Display::fmt(error, f),
        }
    }
}

impl From<std::io::Error> for GetExtError {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}

impl From<std::string::FromUtf8Error> for GetExtError {
    fn from(error: std::string::FromUtf8Error) -> Self {
        Self::FromUtf8Error(error)
    }
}

impl From<serde_json::Error> for GetExtError {
    fn from(error: serde_json::Error) -> Self {
        Self::JsonError(error)
    }
}

pub(crate) fn get_installed_extensions(vscode_exe: &Path) -> Result<HashSet<String>, GetExtError> {
    trace!("[enter] get_installed_extensions");
    let mut command = create_vs_code_command(vscode_exe, &["--list-extensions"]);
    let output = command.output()?;
    let stdout = String::from_utf8(output.stdout)?;
    debug!("stdout = {}", stdout);
    let extensions = stdout.trim().split_whitespace().map(ToOwned::to_owned).collect();
    trace!("[exit] get_installed_extensions");
    Ok(extensions)
}

pub(crate) enum InstallExtError {
    IoError(std::io::Error),
    ExecutionFailed {
        stdout: String, stderr: String,
    }
}

impl From<std::io::Error> for InstallExtError {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error)
    }
}

pub(crate) fn install_extension(vscode_exe: &Path, extension: &str) -> Result<(), InstallExtError> {
    trace!("[enter] install_extension({:?}, {:?})",
        vscode_exe,
        extension
    );
    let output =
        create_vs_code_command(vscode_exe, &["", "--install-extension", extension])
            .output()?;
    debug!("stdout raw: {:?}", &output.stdout);
    let stdout = String::from_utf8_lossy(&output.stdout);
    debug!("stdout:\n{:?}", stdout);
    debug!("stderr raw: {:?}", &output.stderr);
    let stderr = String::from_utf8_lossy(&output.stderr);
    debug!("stderr:\n{:?}", stderr);
    trace!("[exit] install_extension");
    if output.status.success() {
        Ok(())
    } else {
        Err(InstallExtError::ExecutionFailed {
            stdout: stdout.to_string(), stderr: stderr.to_string(),
        })
    }
}

fn get_vscode_home() -> Result<PathBuf, GetExtError> {
    let home_dir = dirs::home_dir().ok_or(GetExtError::NoHomeDir)?;
    debug!("home_dir = {:?}", home_dir);
    let mut vscode_home = home_dir;
    vscode_home.push(".vscode");
    Ok(vscode_home)
}

pub(crate) fn get_installed_extensions_quick() -> Result<HashSet<String>, GetExtError> {
    let mut vs_code_extensions_dir = get_vscode_home()?;
    vs_code_extensions_dir.push("extensions");
    debug!(
        "vs_code_extensions_dir = {:?}",
        vs_code_extensions_dir
    );
    let extensions_pattern = vs_code_extensions_dir.join("*");
    debug!("extensions_pattern = {:?}", extensions_pattern);
    let mut installed_extensions = HashSet::new();
    if let Some(pattern_as_str) = extensions_pattern.to_str() {
        debug!("pattern_as_str = {:?}", pattern_as_str);
        if let Ok(paths) = glob::glob(pattern_as_str) {
            for entry in paths {
                match entry {
                    Ok(path) => {
                        debug!("entry.path = {:?}", path);
                        if let Some(file_name) = path.file_name() {
                            debug!("  file_name = {:?}", file_name);
                            if let Some(file_name) = file_name.to_str() {
                                let mut iter = file_name.rsplitn(2, "-");
                                debug!("  dropped part: {:?}", iter.next());
                                if let Some(extension) = iter.next() {
                                    debug!("  found extension: {:?}", extension);
                                    installed_extensions.insert(extension.to_string());
                                }
                            }
                        }
                    }
                    Err(error) => debug!("error for glob entry: {}", error),
                }
            }
        }
    }
    Ok(installed_extensions)
}

pub(crate) fn set_locale(new_locale: &str) -> Result<(), GetExtError> {
    trace!("[enter] set_locale(new_locale={})", new_locale);
    let mut vscode_argv = get_vscode_home()?;
    vscode_argv.push("argv.json");
    let json: Option<serde_json::Value> = if let Ok(reader) = File::open(&vscode_argv) {
        Some(serde_json::from_reader(reader)?)
    } else {
        None
    };
    let updated_json = match json {
        Some(mut value) => {
            debug!("Initial json: {}", value);
            match &mut value {
                serde_json::Value::Object(map) => {
                    let old = map.insert("locale".to_string(), serde_json::Value::String(new_locale.to_string()));
                    debug!("old locale: {:?}", old);
                }
                _ => {
                    return Err(GetExtError::InvalidJson(vscode_argv));
                }
            }
            value
        }
        None => {
            serde_json::json!({
                "locale": new_locale,
            })
        }
    };
    debug!("Updated json: {}", updated_json);
    let writer = File::create(vscode_argv)?;
    serde_json::to_writer_pretty(writer, &updated_json)?;
    trace!("[exit] set_locale");
    Ok(())
}

pub(crate) fn enable_setting_if_unset(setting: &str) -> Result<(), GetExtError> {
    trace!("[enter] enable_setting_if_unset(setting={})", setting);
    let mut settings_path = dirs::config_dir().ok_or(GetExtError::NoConfigDir)?;
    settings_path.push("Code");
    settings_path.push("User");
    settings_path.push("settings.json");
    let json: Option<serde_json::Value> = if let Ok(reader) = File::open(&settings_path) {
        Some(serde_json::from_reader(reader)?)
    } else {
        None
    };
    let updated_json = match json {
        Some(mut value) => {
            debug!("Initial json: {}", value);
            match &mut value {
                serde_json::Value::Object(map) => {
                    if !map.contains_key(setting) {
                        map.insert(setting.to_string(), serde_json::Value::Bool(true));
                    } else {
                        debug!("The map already has key: {}", setting);
                    }
                }
                _ => {
                    return Err(GetExtError::InvalidJson(settings_path));
                }
            }
            value
        }
        None => {
            serde_json::json!({
                setting: true,
            })
        }
    };
    debug!("Updated json: {}", updated_json);
    let writer = File::create(settings_path)?;
    serde_json::to_writer_pretty(writer, &updated_json)?;
    trace!("[exit] enable_setting_if_unset");
    Ok(())
}