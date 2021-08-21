use std::{path::{Path, }, sync::{Arc}};

use log::{trace, error};
use nwd::NwgUi;
use nwg::{NativeUi, NwgError};

use super::{Extension, extension::{ExtensionInstallerList}};

#[derive(NwgUi)]
pub struct ExtInstallApp {
    extensions: Arc<ExtensionInstallerList>,

    #[nwg_control(size: (590, 430), position: (300, 300), title: "Smauglys: diegiami papildiniai", flags: "WINDOW|VISIBLE")]
    #[nwg_events(OnWindowClose: [ExtInstallApp::exit], OnInit: [ExtInstallApp::init_text], OnMinMaxInfo: [ExtInstallApp::set_resize(SELF, EVT_DATA)] )]
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
        if let Some(abort_message) = self.extensions.get_abort_message() {
            nwg::modal_error_message(&self.window, "Kritinė klaida", &abort_message);
            nwg::stop_thread_dispatch();
        } else {
        if self.extensions.is_finished() {
            nwg::stop_thread_dispatch();
        } else {
            let mut buf = String::from("Diegiami papildiniai:\r\n");
            self.extensions.get_state(&mut buf);
            self.explanation.set_text(&*buf);
        }
        }
    }

    fn init_text(&self) {
        self.render_state();
    }

    fn set_resize(&self, data: &nwg::EventData) {
        let data = data.on_min_max();
        data.set_min_size(200, 200);
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
        unimplemented!("TODO: Stop the spinning thread.")
    }

}

fn do_run(vscode_exe: &Path, extensions: &[Extension]) -> Result<(), NwgError> {
    trace!("[enter] gui::run");
    nwg::init()?;
    nwg::Font::set_global_family("Segoe UI")?;
    let initial_state = ExtInstallApp {
        extensions: Arc::new(ExtensionInstallerList::new(vscode_exe, extensions)),
        window: Default::default(),
        grid: Default::default(),
        text_font: Default::default(),
        explanation: Default::default(),
        notice: Default::default(),
    };
    let app = ExtInstallApp::build_ui(initial_state)?;
    let sender = app.notice.sender();
    let extensions_installer = app.extensions.clone();
    let _thread = std::thread::spawn(move || {
        while !extensions_installer.is_finished() {
            sender.notice();
            extensions_installer.process_next_action();
            sender.notice();
        }
    });
    nwg::dispatch_thread_events();
    trace!("[exit] gui::run");
    Ok(())
}

pub(super) fn run(vscode_exe: &Path, extensions: &[Extension]) {
    trace!("[enter] gui::run");
    match do_run(vscode_exe, extensions) {
        Ok(()) => (),
        Err(error) => {
            error!("Error occurred when starting GUI: {}", error);
        }
    }
    trace!("[exit] gui::run");
}