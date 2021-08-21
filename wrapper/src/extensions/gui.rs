use std::{path::{Path, PathBuf}, };

use log::{trace, error};
use nwd::NwgUi;
use nwg::{NativeUi, NwgError};

use super::{Extension, extension::ExtensionInstaller};

#[derive(NwgUi)]
pub struct ExtInstallApp {
    vscode_exe: PathBuf,
    extensions: Vec<ExtensionInstaller>,
    counter: std::sync::Arc<std::sync::atomic::AtomicU64>,

    #[nwg_control(size: (590, 430), position: (300, 300), title: "Smauglys: diegiami papildiniai", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [ExtInstallApp::say_goodbye], OnInit: [ExtInstallApp::init_text], OnMinMaxInfo: [ExtInstallApp::set_resize(SELF, EVT_DATA)] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 1)]
    grid: nwg::GridLayout,

    #[nwg_resource(family: "Segoe UI", size: 18)]
    text_font: nwg::Font,

    #[nwg_control(font: Some(&data.text_font), flags: "VISIBLE|MULTI_LINE")]
    #[nwg_layout_item(layout: grid, row: 0, col: 0)]
    explanation: nwg::RichLabel,

    #[nwg_control(parent: window)]
    #[nwg_events(OnNotice: [ExtInstallApp::render_state])]
    notice: nwg::Notice,
}

impl ExtInstallApp {

    fn render_state(&self) {
        let mut buf = String::from("Diegiami papildiniai:\r\n");
        for extension in &self.extensions {
            buf.push_str(&extension.state_as_line());
        }
        buf.push_str(&format!("current counter: {}", self.counter.load(std::sync::atomic::Ordering::SeqCst)));
        eprintln!("in render state");
        self.explanation.set_text(&*buf);
    }

    fn init_text(&self) {
        self.render_state();
    }

    fn set_resize(&self, data: &nwg::EventData) {
        let data = data.on_min_max();
        data.set_min_size(200, 200);
    }

    fn say_goodbye(&self) {
        nwg::modal_info_message(&self.window, "Goodbye", &format!("Goodbye someone"));
        nwg::stop_thread_dispatch();
    }

}

fn do_run(vscode_exe: &Path, extensions: &[Extension]) -> Result<(), NwgError> {
    trace!("[enter] gui::run");
    nwg::init()?;
    nwg::Font::set_global_family("Segoe UI")?;
    let initial_state = ExtInstallApp {
        vscode_exe: vscode_exe.to_owned(),
        extensions: extensions.iter().map(|extension| extension.into()).collect(),
        counter: Default::default(),
        window: Default::default(),
        grid: Default::default(),
        text_font: Default::default(),
        explanation: Default::default(),
        notice: Default::default(),
    };
    let sender = initial_state.notice.sender();
    let counter = initial_state.counter.clone();
    let _thread = std::thread::spawn(move || {
        loop {
            eprintln!("in spin loop");
            counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            sender.notice();
            std::thread::sleep(std::time::Duration::new(5, 0));
        }
    });
    let _app = ExtInstallApp::build_ui(initial_state)?;
    eprintln!("before dispatch loop");
    nwg::dispatch_thread_events();
    trace!("[exit] gui::run");
    Ok(())
}

pub(super) fn run(vscode_exe: &Path, extensions: &[Extension]) {
    trace!("[enter] gui::run");
    eprintln!("[enter] gui::run");
    match do_run(vscode_exe, extensions) {
        Ok(()) => (),
        Err(error) => {
            error!("Error occurred when starting GUI: {}", error);
        }
    }
    trace!("[exit] gui::run");
}