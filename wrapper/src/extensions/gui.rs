use std::{path::{Path, }, sync::{mpsc::{Receiver, TryRecvError, channel}}};

use log::{debug, error, trace};
use nwd::NwgUi;
use nwg::{NativeUi, NwgError};

use super::{Extension, extension::{ExtensionInstallerList}};

enum Message {
    UiUpdate {
        progress: u32,
        progress_total: u32,
        details: String,
    },
    Finished,
    Abort(String),
}

#[derive(NwgUi)]
pub struct ExtInstallApp {
    receiver: Receiver<Message>,

    #[nwg_control(size: (590, 430), position: (300, 300), title: "Smauglys: diegiami papildiniai", flags: "WINDOW|VISIBLE")]
    #[nwg_events(OnWindowClose: [ExtInstallApp::exit], OnInit: [ExtInstallApp::init_text], OnMinMaxInfo: [ExtInstallApp::set_resize(SELF, EVT_DATA)] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 1)]
    grid: nwg::GridLayout,

    #[nwg_control(step: 1, range: 0..1)]
    #[nwg_layout_item(layout: grid, row: 0, col: 0)]
    progress_bar: nwg::ProgressBar,

    #[nwg_resource(family: "Segoe UI", size: 18)]
    text_font: nwg::Font,

    #[nwg_control(font: Some(&data.text_font), flags: "VISIBLE|MULTI_LINE")]
    #[nwg_layout_item(layout: grid, row: 1, col: 0)]
    explanation: nwg::RichLabel,

    #[nwg_control(parent: window)]
    #[nwg_events(OnNotice: [ExtInstallApp::render_state])]
    notice: nwg::Notice,
}

impl ExtInstallApp {

    fn render_state(&self) {
        trace!("[enter] render_state");
        match self.receiver.try_recv() {
            Ok(Message::UiUpdate { progress, progress_total, details }) => {
                self.progress_bar.set_range(0..progress_total);
                self.progress_bar.set_pos(progress);
                self.explanation.set_text(&details);
            }
            Ok(Message::Finished) => {
                nwg::stop_thread_dispatch();
            }
            Ok(Message::Abort(abort_message)) => {
                error!("critical error: {}", abort_message);
                nwg::modal_error_message(&self.window, "Kritinė klaida", &abort_message);
                nwg::stop_thread_dispatch();
            }
            Err(TryRecvError::Disconnected) => {
                error!("Disconnected channel.");
                nwg::stop_thread_dispatch();
            }
            Err(TryRecvError::Empty) => {
                debug!("empty channel");
            }
        }
        trace!("[exit] render_state");
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
    let (sender, receiver) = channel();
    let initial_state = ExtInstallApp {
        receiver,
        window: Default::default(),
        grid: Default::default(),
        progress_bar: Default::default(),
        text_font: Default::default(),
        explanation: Default::default(),
        notice: Default::default(),
    };
    let app = ExtInstallApp::build_ui(initial_state)?;
    let notice_sender = app.notice.sender();
    let extension_count = extensions.len();
    let extensions_installer = ExtensionInstallerList::new(vscode_exe, extensions);
    let _thread = std::thread::spawn(move || {
        while !extensions_installer.is_finished() {
            notice_sender.notice();
            trace!("Sent GUI notification.");
            extensions_installer.process_next_action();
            if let Some(abort_message) = extensions_installer.get_abort_message() {
                sender.send(Message::Abort(abort_message)).unwrap();
                notice_sender.notice();
                break;
            } else {
                let mut buf = String::from("Diegiami papildiniai:\r\n");
                extensions_installer.get_state(&mut buf);
                sender.send(Message::UiUpdate {
                    progress: extensions_installer.get_current_progress(),
                    progress_total: extension_count as u32,
                    details: buf
                }).unwrap();
                notice_sender.notice();
            }
            trace!("Sent GUI notification.");
        }
        if let Ok(()) = sender.send(Message::Finished) {
            notice_sender.notice();
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