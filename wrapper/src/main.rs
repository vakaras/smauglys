use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::Command;

const EXTENSIONS: &'static [&'static str] = &["ms-python.python"];

type Logger = Option<File>;

macro_rules! log {
    ( $self:ident, $( $x:expr ),* ) => {
        if let Some(logger) = $self {
            writeln!(logger, $( $x ),*).unwrap();
        }
    }
}

fn log_raw(logger: &mut Logger, bytes: &[u8]) {
    if let Some(file) = logger {
        file.write_all(bytes).unwrap();
    }
}

fn get_vscode_original_exe() -> PathBuf {
    let current_exe = std::env::current_exe().expect("Failed to detect current exe.");
    let mut code_original_exe = current_exe;
    code_original_exe.set_file_name("code_original.exe");
    code_original_exe
}

fn create_vs_code_command(logger: &mut Logger, vscode_exe: &Path, args: &[&str]) -> Command {
    log!(
        logger,
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
    log!(logger, "[exit] create_vs_code_command");
    command
}

fn install_extension(
    logger: &mut Logger,
    vscode_exe: &Path,
    extension: &str,
) -> std::io::Result<()> {
    log!(
        logger,
        "[enter] install_extension({:?}, {:?})",
        vscode_exe,
        extension
    );
    let output =
        create_vs_code_command(logger, vscode_exe, &["", "--install-extension", extension])
            .output()?;
    log!(logger, "stdout:");
    log_raw(logger, &output.stdout);
    log!(logger, "stderr:");
    log_raw(logger, &output.stderr);
    log!(logger, "[exit] install_extension");
    Ok(())
}

fn ensure_extensions(logger: &mut Logger, vscode_exe: &Path) {
    log!(
        logger,
        "[enter] ensure_extensions(vscode_exe={:?})",
        vscode_exe
    );
    let home_dir = dirs::home_dir().unwrap();
    log!(logger, "home_dir = {:?}", home_dir);
    let mut vs_code_extensions_dir = home_dir;
    vs_code_extensions_dir.push(".vscode");
    vs_code_extensions_dir.push("extensions");
    log!(
        logger,
        "vs_code_extensions_dir = {:?}",
        vs_code_extensions_dir
    );
    let extensions_pattern = vs_code_extensions_dir.join("*");
    log!(logger, "extensions_pattern = {:?}", extensions_pattern);
    let mut installed_extensions = HashSet::new();
    if let Some(pattern_as_str) = extensions_pattern.to_str() {
        log!(logger, "pattern_as_str = {:?}", pattern_as_str);
        if let Ok(paths) = glob::glob(pattern_as_str) {
            for entry in paths {
                match entry {
                    Ok(path) => {
                        log!(logger, "entry.path = {:?}", path);
                        if let Some(file_name) = path.file_name() {
                            log!(logger, "  file_name = {:?}", file_name);
                            if let Some(file_name) = file_name.to_str() {
                                let parts: Vec<_> = file_name.splitn(3, "-").collect();
                                let extension = format!("{}-{}", parts[0], parts[1]);
                                log!(logger, "  found extension: {:?}", extension);
                                installed_extensions.insert(extension);
                            }
                        }
                    }
                    Err(error) => log!(logger, "error for glob entry: {}", error),
                }
            }
        }
    }
    for extension in EXTENSIONS {
        log!(logger, "checking extension: {:?}", extension);
        if !installed_extensions.contains(*extension) {
            log!(logger, "  not installed");
            if let Err(error) = install_extension(logger, vscode_exe, extension) {
                log!(
                    logger,
                    "Error installing extension {}: {}",
                    extension,
                    error
                );
            }
        }
    }
    log!(logger, "[exit] ensure_extensions");
}

fn start_vs_code(logger: &mut Logger, vscode_exe: &Path) -> std::io::Result<()> {
    log!(logger, "[enter] start_vs_code(vscode_exe={:?})", vscode_exe);
    let mut args = std::env::args();
    log!(logger, "first arg: {:?}", args.next());
    let exit_status = Command::new(vscode_exe).args(args).status()?;
    log!(logger, "exit status: {}", exit_status);
    log!(logger, "[exit] start_vs_code");
    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut logger = if let Ok(path) = std::env::var("CODE_WRAPPER_LOG_PATH") {
        Some(File::create(path).unwrap())
    } else {
        None
    };
    let vscode_exe = get_vscode_original_exe();
    ensure_extensions(&mut logger, &vscode_exe);
    start_vs_code(&mut logger, &vscode_exe)
}
