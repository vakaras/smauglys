#![windows_subsystem = "windows"]

use log::debug;

use extensions::Extension;

extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

const EXTENSIONS: [Extension; 3] = [
    extensions::PYTHON_EXTENSION,
    extensions::VSCODE_LANGUAGE_PACK_LT_EXTENSION,
    extensions::DEBUG_VISUALIZER_EXTENSION,
];

mod code;
mod extensions;

fn main() -> std::io::Result<()> {
    env_logger::init();
    eprintln!("RUST_LOG={:?}", std::env::var("RUST_LOG"));
    eprintln!("RUST_LOG_STYLE={:?}", std::env::var("RUST_LOG_STYLE"));
    log::error!("Logging is initialized");
    log::info!("Logging is initialized");
    log::debug!("Logging is initialized");
    log::trace!("Logging is initialized");
    let vscode_exe = code::get_vscode_original_exe();
    debug!("vscode_exe: {:?}", vscode_exe);
    extensions::ensure_installed(&vscode_exe, &EXTENSIONS);
    code::start_vs_code(&vscode_exe)
}


// use nwd::NwgUi;
// use nwg::NativeUi;
// #[derive(Default, NwgUi)]
// pub struct ExtApp {
//     #[nwg_control(size: (590, 430), position: (300, 300), title: "Basic example", flags: "WINDOW|VISIBLE")]
//     #[nwg_events( OnWindowClose: [ExtApp::say_goodbye], OnInit: [ExtApp::init_text], OnMinMaxInfo: [ExtApp::set_resize(SELF, EVT_DATA)] )]
//     window: nwg::Window,

//     #[nwg_layout(parent: window, spacing: 1)]
//     grid: nwg::GridLayout,

//     #[nwg_resource(family: "Segoe UI", size: 18)]
//     text_font: nwg::Font,

//     #[nwg_control(font: Some(&data.text_font), flags: "VISIBLE|MULTI_LINE")]
//     #[nwg_layout_item(layout: grid, row: 0, col: 0)]
//     explanation: nwg::RichLabel,
// }

// impl ExtApp {

//     fn init_text(&self) {
//         let text = concat!(
//             "Lietuviškų raidžių testas.\r\n",
//             "ąčęėšųū„“\r\n",
//             "ĄČĘĖĮŠŲŪ“”\r\n",
//         );
//         self.explanation.set_text(text);
//     }

//     fn set_resize(&self, data: &nwg::EventData) {
//         let data = data.on_min_max();
//         data.set_min_size(200, 200);
//     }

//     fn say_goodbye(&self) {
//         nwg::modal_info_message(&self.window, "Goodbye", &format!("Goodbye someone"));
//         nwg::stop_thread_dispatch();
//     }

// }

// fn main() {
//     nwg::init().expect("Failed to init Native Windows GUI");
//     nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
//     let _app = ExtApp::build_ui(Default::default()).expect("Failed to build UI");
//     nwg::dispatch_thread_events();
// }

// use std::collections::HashSet;
// use std::fs::File;
// use std::io::prelude::*;
// use std::path::{Path, PathBuf};
// use std::process::Command;

// macro_rules! log {
//     ( $self:ident, $( $x:expr ),* ) => {
//         if let Some(logger) = $self {
//             writeln!(logger, $( $x ),*).unwrap();
//         }
//     }
// }

// // mod extension_installer;

// const EXTENSIONS: &'static [&'static str] = &[
//     "ms-python.python",
//     "vakaras.vscode-language-pack-lt",
//     "hediet.debug-visualizer",
// ];

// type Logger = Option<File>;

// fn log_raw(logger: &mut Logger, bytes: &[u8]) {
//     if let Some(file) = logger {
//         file.write_all(bytes).unwrap();
//     }
// }

// fn get_vscode_original_exe() -> PathBuf {
//     let current_exe = std::env::current_exe().expect("Failed to detect current exe.");
//     let mut code_original_exe = current_exe;
//     code_original_exe.set_file_name("code_original.exe");
//     code_original_exe
// }

// fn create_vs_code_command(logger: &mut Logger, vscode_exe: &Path, args: &[&str]) -> Command {
//     log!(
//         logger,
//         "[enter] create_vs_code_command(vscode_exe={:?}, args={:?})",
//         vscode_exe,
//         args
//     );
//     let mut cli_path = vscode_exe.to_path_buf();
//     cli_path.pop();
//     cli_path.push("resources");
//     cli_path.push("app");
//     cli_path.push("out");
//     cli_path.push("cli.js");

//     let mut command = Command::new(vscode_exe);
//     command
//         .env("ELECTRON_RUN_AS_NODE", "1")
//         .arg(cli_path)
//         .args(args);
//     log!(logger, "[exit] create_vs_code_command");
//     command
// }

// fn install_extension(
//     logger: &mut Logger,
//     vscode_exe: &Path,
//     extension: &str,
// ) -> std::io::Result<()> {
//     log!(
//         logger,
//         "[enter] install_extension({:?}, {:?})",
//         vscode_exe,
//         extension
//     );
//     let output =
//         create_vs_code_command(logger, vscode_exe, &["", "--install-extension", extension])
//             .output()?;
//     log!(logger, "stdout:");
//     log_raw(logger, &output.stdout);
//     log!(logger, "stderr:");
//     log_raw(logger, &output.stderr);
//     log!(logger, "[exit] install_extension");
//     Ok(())
// }

// fn install_extensions(logger: &mut Logger, vscode_exe: PathBuf, extensions_to_install: Vec<&'static str>) {
//     log!(
//         logger,
//         "[enter] install_extensions({:?}, {:?})",
//         vscode_exe,
//         extensions_to_install
//     );
//     // TODO: Should launch VS Code in the end.
//     // let flags = extension_installer::Flags {
//     //     extensions_to_install,
//     //     vscode_exe,
//     // };
//     eprintln!("Starting GUI");

//     // TODO: After installing the extensions, spawn the VS Code processes and process::exit.
//     // if cfg!(windows) {
//         // if std::env::var("WGPU_BACKEND").is_err() {
//         //     // WGPU_BACKEND is not set. Set to DX11.
//         //     std::env::set_var("WGPU_BACKEND", "dx11");
//         //     eprintln!("Updated WGPU_BACKEND to dx11");
//         // }
//     // }
//     // let settings = iced::Settings::with_flags(flags);
//     // <extension_installer::ExtensionInstaller as iced::Application>::run(settings).unwrap();
//     // for extension in extensions_to_install {
//     //     if let Err(error) = install_extension(logger, vscode_exe, extension) {
//     //         log!(
//     //             logger,
//     //             "Error installing extension {}: {}",
//     //             extension,
//     //             error
//     //         );
//     //     }
//     // }
//     log!(logger, "[exit] install_extensions");
// }

// fn ensure_extensions(logger: &mut Logger, vscode_exe: &Path) {
//     log!(
//         logger,
//         "[enter] ensure_extensions(vscode_exe={:?})",
//         vscode_exe
//     );
//     let home_dir = dirs::home_dir().unwrap();
//     log!(logger, "home_dir = {:?}", home_dir);
//     let mut vs_code_extensions_dir = home_dir;
//     vs_code_extensions_dir.push(".vscode");
//     vs_code_extensions_dir.push("extensions");
//     log!(
//         logger,
//         "vs_code_extensions_dir = {:?}",
//         vs_code_extensions_dir
//     );
//     let extensions_pattern = vs_code_extensions_dir.join("*");
//     log!(logger, "extensions_pattern = {:?}", extensions_pattern);
//     let mut installed_extensions = HashSet::new();
//     if let Some(pattern_as_str) = extensions_pattern.to_str() {
//         log!(logger, "pattern_as_str = {:?}", pattern_as_str);
//         if let Ok(paths) = glob::glob(pattern_as_str) {
//             for entry in paths {
//                 match entry {
//                     Ok(path) => {
//                         log!(logger, "entry.path = {:?}", path);
//                         if let Some(file_name) = path.file_name() {
//                             log!(logger, "  file_name = {:?}", file_name);
//                             if let Some(file_name) = file_name.to_str() {
//                                 let parts: Vec<_> = file_name.splitn(3, "-").collect();
//                                 if let (Some(publisher), Some(name)) = (parts.get(0), parts.get(1))
//                                 {
//                                     let extension = format!("{}-{}", publisher, name);
//                                     log!(logger, "  found extension: {:?}", extension);
//                                     installed_extensions.insert(extension);
//                                 }
//                             }
//                         }
//                     }
//                     Err(error) => log!(logger, "error for glob entry: {}", error),
//                 }
//             }
//         }
//     }
//     let mut extensions_to_install = Vec::new();
//     for extension in EXTENSIONS {
//         log!(logger, "checking extension: {:?}", extension);
//         if installed_extensions.contains(*extension) {
//             log!(logger, "  already installed");
//         } else {
//             log!(logger, "  not installed");
//             extensions_to_install.push(*extension);
//         }
//     }
//     if extensions_to_install.is_empty() {
//         log!(logger, "No extensions to install");
//     } else {
//         log!(
//             logger,
//             "Installing {} extensions",
//             extensions_to_install.len()
//         );
//         install_extensions(logger, vscode_exe.to_path_buf(), extensions_to_install);
//     }
//     log!(logger, "[exit] ensure_extensions");
// }

// fn start_vs_code(logger: &mut Logger, vscode_exe: &Path) -> std::io::Result<()> {
//     log!(logger, "[enter] start_vs_code(vscode_exe={:?})", vscode_exe);
//     let mut args = std::env::args();
//     log!(logger, "first arg: {:?}", args.next());
//     let exit_status = Command::new(vscode_exe).args(args).status()?;
//     log!(logger, "exit status: {}", exit_status);
//     log!(logger, "[exit] start_vs_code");
//     Ok(())
// }

// fn main() -> std::io::Result<()> {
//     let mut logger = if let Ok(path) = std::env::var("CODE_WRAPPER_LOG_PATH") {
//         Some(File::create(path).unwrap())
//     } else {
//         None
//     };
//     let vscode_exe = get_vscode_original_exe();
//     ensure_extensions(&mut logger, &vscode_exe);
//     start_vs_code(&mut logger, &vscode_exe)
// }
