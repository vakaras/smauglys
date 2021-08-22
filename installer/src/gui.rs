use std::sync::mpsc::{Receiver, TryRecvError};

use log::{debug, error, trace};
use nwd::NwgUi;
use nwg::{NativeUi};

use crate::error::IResult;

enum Message {
    ProgressUpdate {
        progress: u32,
        details: String,
    },
    Finished,
    Abort(String),
}

#[derive(NwgUi)]
pub struct InstallerApp {

    #[nwg_control(size: (530, 300), position: (300, 300), title: "Smauglys: diegiami papildiniai", flags: "WINDOW|VISIBLE")]
    #[nwg_events(OnWindowClose: [InstallerApp::exit], OnInit: [InstallerApp::show_initial_wiew], OnMinMaxInfo: [InstallerApp::set_resize(SELF, EVT_DATA)] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 1)]
    grid_initial: nwg::GridLayout,

    #[nwg_control(text: "Inicializuojama.", flags: "VISIBLE|MULTI_LINE")]
    #[nwg_layout_item(layout: grid_initial, row: 0, col: 0, row_span: 4)]
    explanation: nwg::RichLabel,

    #[nwg_control(text: "Inicializuojama.")]
    #[nwg_layout_item(layout: grid_initial, row: 5, col: 0)]
    python_license_checkbox: nwg::CheckBox,

    #[nwg_control(text: "Inicializuojama.")]
    #[nwg_layout_item(layout: grid_initial, row: 6, col: 0)]
    vscode_license_checkbox: nwg::CheckBox,

    #[nwg_control(text: "Įdiegti")]
    #[nwg_layout_item(layout: grid_initial, row: 7, col: 0)]
    #[nwg_events( OnButtonClick: [InstallerApp::show_progress_view] )]
    install_button: nwg::Button,

    #[nwg_layout(parent: window, spacing: 1)]
    grid_installing: nwg::GridLayout,

    #[nwg_control(parent: window)]
    #[nwg_events(OnNotice: [InstallerApp::update_progress_bar])]
    progress_bar_notice: nwg::Notice,

    #[nwg_control(step: 1, range: 0..3)]
    #[nwg_layout_item(layout: grid_installing, row: 1, col: 0)]
    progress_bar: nwg::ProgressBar,

    #[nwg_control(text: "Inicializuojama.", flags: "VISIBLE|MULTI_LINE")]
    #[nwg_layout_item(layout: grid_installing, row: 0, col: 0)]
    progress_bar_details: nwg::RichLabel,

    progress_bar_receiver: Option<Receiver<Message>>,

    #[nwg_layout(parent: window, spacing: 1)]
    grid_final: nwg::GridLayout,
}

impl InstallerApp {
    fn update_progress_bar(&self) {
        trace!("[enter] update_progress_bar");
        if let Some(receiver) = &self.progress_bar_receiver {
            match receiver.try_recv() {
                Ok(Message::ProgressUpdate {
                    progress,
                    details,
                }) => {
                    self.progress_bar.set_pos(progress);
                    self.progress_bar_details.set_text(&details);
                }
                Ok(Message::Finished) => {
                    self.show_final_wiew();
                }
                Ok(Message::Abort(abort_message)) => {
                    error!("critical error: {}", abort_message);
                    nwg::modal_error_message(&self.window, "Kritinė klaida", &abort_message);
                    nwg::stop_thread_dispatch();
                }
                Err(TryRecvError::Disconnected) => {
                    error!("Disconnected channel.");
                    self.show_final_wiew();
                }
                Err(TryRecvError::Empty) => {
                    debug!("empty channel");
                }
            }
        } else {
            error!("Internal error: progress_bar_receiver is None");
        }
        trace!("[exit] update_progress_bar");
    }
    fn set_resize(&self, data: &nwg::EventData) {
        let data = data.on_min_max();
        data.set_min_size(200, 200);
    }
    fn exit(&self) {
        nwg::stop_thread_dispatch();
        error!("TODO: stop the spinning thread.");
        unimplemented!("TODO: Stop the spinning thread.")
    }
    fn set_visible_initial_view(&self, visible: bool) {
        self.explanation.set_visible(visible);
        self.python_license_checkbox.set_visible(visible);
        self.vscode_license_checkbox.set_visible(visible);
        self.install_button.set_visible(visible);
    }
    fn set_visible_progress_view(&self, visible: bool) {
        self.progress_bar.set_visible(visible);
        self.progress_bar_details.set_visible(visible);
    }
    fn set_visible_final_wiew(&self, visible: bool) {
    }
    fn show_initial_wiew(&self) {
        trace!("[enter] show_initial_wiew");
        self.set_visible_initial_view(true);
        self.set_visible_progress_view(false);
        self.set_visible_final_wiew(false);
        trace!("[exit] show_initial_wiew");
    }
    fn show_progress_view(&self) {
        trace!("[enter] show_progress_view");
        self.set_visible_initial_view(false);
        self.set_visible_progress_view(true);
        self.set_visible_final_wiew(false);
        trace!("[exit] show_progress_view");
    }
    fn show_final_wiew(&self) {
        trace!("[enter] show_final_wiew");
        self.set_visible_initial_view(false);
        self.set_visible_progress_view(false);
        self.set_visible_final_wiew(true);
        trace!("[exit] show_final_wiew");
    }
}

pub(crate) fn run() -> IResult {
    trace!("[enter] gui::run");
    nwg::init()?;
    let mut font = nwg::Font::default();
    nwg::Font::builder().size(18).family("Segoe UI").build(&mut font)?;
    nwg::Font::set_global_default(Some(font));
    let initial_state = InstallerApp {
        progress_bar_receiver: None,
        window: Default::default(),
        grid_initial: Default::default(),
        python_license_checkbox: Default::default(),
        vscode_license_checkbox: Default::default(),
        install_button: Default::default(),
        grid_installing: Default::default(),
        progress_bar_notice: Default::default(),
        progress_bar: Default::default(),
        progress_bar_details: Default::default(),
        grid_final: Default::default(),
        explanation: Default::default(),
    };
    let _app = InstallerApp::build_ui(initial_state)?;
    nwg::dispatch_thread_events();
    // let state = State::default();
    // extract_file(PYTHON_INSTALLER, &state.python_installer).unwrap();
    // extract_file(VSCODE_INSTALLER, &state.vscode_installer).unwrap();
    // extract_file(WRAPPER_BIN, &state.wrapper_bin).unwrap();
    // install_python(&state.python_installer).unwrap();
    // install_vscode(&state.vscode_installer).unwrap();
    trace!("[exit] gui::run");
    Ok(())
}