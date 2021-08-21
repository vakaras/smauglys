use crate::Logger;
use iced::{Align, Application, Button, Clipboard, Color, Column, Command, Container, Element, Length, ProgressBar, Settings, Subscription, Text, button, executor};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

fn create_vs_code_command(
    logger: &mut Logger,
    vscode_exe: &Path,
    args: &[&str],
) -> tokio::process::Command {
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

    // let mut command = tokio::process::Command::new(vscode_exe);
    // command
    //     .env("ELECTRON_RUN_AS_NODE", "1")
    //     .arg(cli_path)
    //     .args(args);
    let mut command = tokio::process::Command::new("ls");
    eprintln!("Sleeping");
    std::thread::sleep(std::time::Duration::from_secs(5));
    log!(logger, "[exit] create_vs_code_command");
    command
}

fn create_install_extension_command(
    logger: &mut Logger,
    vscode_exe: &Path,
    extension: &str,
) -> impl std::future::Future<Output = std::io::Result<std::process::Output>> {
    log!(
        logger,
        "[enter] install_extension({:?}, {:?})",
        vscode_exe,
        extension
    );
    create_vs_code_command(logger, vscode_exe, &["", "--install-extension", extension]).output()
}

fn handle_installation_result(result: std::io::Result<std::process::Output>) -> Message {
    // TODO
    Message::ExtensionInstalled
}

#[derive(Debug)]
pub struct ExtensionInstaller {
    extensions_to_install: Vec<&'static str>,
    currently_installing: usize,
    vscode_exe: PathBuf,
}

#[derive(Debug, Clone)]
pub enum Message {
    ExtensionInstalled,
}

pub struct Flags {
    pub extensions_to_install: Vec<&'static str>,
    pub vscode_exe: PathBuf,
}

impl ExtensionInstaller {
    fn next_command(&self) -> Command<Message> {
        eprintln!("Next command");
        Command::perform(
            create_install_extension_command(
                &mut None,
                &self.vscode_exe,
                &self.extensions_to_install[self.currently_installing],
            ),
            handle_installation_result,
        )
    }
}

impl Application for ExtensionInstaller {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = Flags;

    fn new(flags: Self::Flags) -> (ExtensionInstaller, Command<Message>) {
        let extensions_to_install = flags.extensions_to_install;
        eprintln!("extensions_to_install: {:?}", extensions_to_install);
        let installer = ExtensionInstaller {
            extensions_to_install,
            currently_installing: 0,
            vscode_exe: flags.vscode_exe,
        };
        let command = installer.next_command();
        (installer, command)
    }

    fn title(&self) -> String {
        String::from("Smauglys – diegiami VS Code papildiniai")
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        eprintln!("update: {}", self.currently_installing);
        match message {
            Message::ExtensionInstalled => {
                // self.currently_installing += 1; FIXME
            }
        };
        if self.currently_installing == self.extensions_to_install.len() {
            // Start VS Code and exit the installer.
            let mut args = std::env::args();
            let _drop_first = args.next();
            std::process::Command::new(&self.vscode_exe).args(args).spawn().unwrap();
            std::process::exit(0);
        } else {
            self.next_command()
        }
    }

    // fn subscription(&self) -> Subscription<Message> {
    //     Subscription::batch(self.installations.iter().map(Installation::subscription))
    // }

    fn view(&mut self) -> Element<Message> {
        // let current_progress =
        //     100.0 * (self.currently_installing as f32) / (self.extensions_to_install.len() as f32);
        // let progress_bar = ProgressBar::new(0.0..=100.0, current_progress);
        // let downloads = self
        //     .downloads
        //     .iter_mut()
        //     .fold(Column::new().spacing(20), |column, download| {
        //         column.push(download.view())
        //     })
        //     .push(
        //         Button::new(&mut self.add, Text::new("Add another download"))
        //             .on_press(Message::Add)
        //             .padding(10),
        //     )
        //     .align_items(Align::End);
        let green = Color::new(0.0, 1.0, 0.0, 1.0);
        let black = Color::new(0.0, 0.0, 0.0, 1.0);
        let yellow = Color::new(1.0, 0.0, 1.0, 1.0);
        let column = Column::new()
            .push(Text::new("Įdiegtas papildinys!".to_string()).color(green))
            .push(Text::new("» Diegiamas papildinys!".to_string()).size(20).color(yellow))
            .push(Text::new("Ruošiamasi diegti papildinį!".to_string()).size(30).color(black))
            .push(Text::new("Lietuviškų raidžių testas: ąčęėįšųū„“!".to_string()).size(10).color(black))
            .push(Text::new("Ruošiamasi diegti papildinį!".to_string()).size(50).color(black));

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .padding(20)
            .into()
    }
}

// #[derive(Debug)]
// struct Installation {
//     extension: &'static str,
//     state: State,
// }

// #[derive(Debug)]
// enum State {
//     Idle { button: button::State },
//     Downloading { progress: f32 },
//     Finished { button: button::State },
//     Errored { button: button::State },
// }

// impl Installation {
//     pub fn new(extension: &'static str) -> Self {
//         Installation {
//             extension,
//             state: State::Idle {
//                 button: button::State::new(),
//             },
//         }
//     }

//     pub fn start(&mut self) {
//         match self.state {
//             State::Idle { .. }
//             | State::Finished { .. }
//             | State::Errored { .. } => {
//                 self.state = State::Downloading { progress: 0.0 };
//             }
//             _ => {}
//         }
//     }

//     pub fn progress(&mut self, new_progress: download::Progress) {
//         match &mut self.state {
//             State::Downloading { progress } => match new_progress {
//                 download::Progress::Started => {
//                     *progress = 0.0;
//                 }
//                 download::Progress::Advanced(percentage) => {
//                     *progress = percentage;
//                 }
//                 download::Progress::Finished => {
//                     self.state = State::Finished {
//                         button: button::State::new(),
//                     }
//                 }
//                 download::Progress::Errored => {
//                     self.state = State::Errored {
//                         button: button::State::new(),
//                     };
//                 }
//             },
//             _ => {}
//         }
//     }

//     pub fn subscription(&self) -> Subscription<Message> {
//         match self.state {
//             State::Downloading { .. } => {
//                 download::file(self.id, "https://speed.hetzner.de/100MB.bin?")
//                     .map(Message::DownloadProgressed)
//             }
//             _ => Subscription::none(),
//         }
//     }

//     pub fn view(&mut self) -> Element<Message> {
//         let current_progress = match &self.state {
//             State::Idle { .. } => 0.0,
//             State::Downloading { progress } => *progress,
//             State::Finished { .. } => 100.0,
//             State::Errored { .. } => 0.0,
//         };

//         let progress_bar = ProgressBar::new(0.0..=100.0, current_progress);

//         let control: Element<_> = match &mut self.state {
//             State::Idle { button } => {
//                 Button::new(button, Text::new("Start the download!"))
//                     .on_press(Message::Download(self.id))
//                     .into()
//             }
//             State::Finished { button } => Column::new()
//                 .spacing(10)
//                 .align_items(Align::Center)
//                 .push(Text::new("Download finished!"))
//                 .push(
//                     Button::new(button, Text::new("Start again"))
//                         .on_press(Message::Download(self.id)),
//                 )
//                 .into(),
//             State::Downloading { .. } => {
//                 Text::new(format!("Downloading... {:.2}%", current_progress))
//                     .into()
//             }
//             State::Errored { button } => Column::new()
//                 .spacing(10)
//                 .align_items(Align::Center)
//                 .push(Text::new("Something went wrong :("))
//                 .push(
//                     Button::new(button, Text::new("Try again"))
//                         .on_press(Message::Download(self.id)),
//                 )
//                 .into(),
//         };

//         Column::new()
//             .spacing(10)
//             .padding(10)
//             .align_items(Align::Center)
//             .push(progress_bar)
//             .push(control)
//             .into()
//     }
// }
