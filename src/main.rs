#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use iced::{
    alignment::{Horizontal, Vertical},
    widget::{
        button, column, container, horizontal_space, pick_list, row, space::Space, text, text_input, toggler,
    },
    Application, Command, Element, Length, Theme,
};
use rand::{seq::SliceRandom, thread_rng};
use std::{io, path::PathBuf, sync::Arc, vec};

use grammar::*;
mod settings;
use settings::*;

fn main() -> iced::Result {
    App::run(iced::Settings::default())
}

#[derive(Debug)]
struct App {
    content: Vec<Entry>,
    current: Option<usize>,
    entry: String,
    error: Option<Error>,
    file: Option<PathBuf>,
    langs: [Lang; 2],
    state: State,
    last_score: f32,
    dark_theme: bool,
    total_score: (f32, usize),
}

impl Default for App {
    fn default() -> Self {
        let mut default_content = vec![
            Entry("yes".into(), "oui".into(), GramClass::Adverb),
            Entry("no".into(), "non".into(), GramClass::Adverb),
            Entry("the work".into(), "le travail".into(), GramClass::Noun),
            Entry("the rust".into(), "la rouille".into(), GramClass::Noun),
            Entry("the solution".into(), "la solution".into(), GramClass::Noun),
            Entry("to rise".into(), "s'élever".into(), GramClass::Verb),
        ];
        default_content.shuffle(&mut thread_rng());
        Self {
            total_score: (0., default_content.len()),
            content: default_content,
            current: Some(0),
            entry: String::new(),
            error: None,
            file: None,
            langs: ["English".into(), "French".into()],
            state: State::WaitUserAnswer,
            last_score: 0.,
            dark_theme: true,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    TextInputChanged(String),
    FileOpened(Result<(PathBuf, Arc<String>), Error>),
    Correction,
    Next,
    None,
    OpenFile,
    OpenSettings,
    Start,
    ThemeSelected,
}

#[derive(Debug, Clone)]
enum Error {
    IoError,
    DialogClosed,
}

#[derive(Debug)]
enum State {
    Correcting,
    WaitUserAnswer,
    NotRunning,
    Settings,
    Starting,
}

impl iced::Application for App {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flag: Self::Flags) -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Vocabulary")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::TextInputChanged(value) => {
                self.entry = value;
                Command::none()
            }
            Message::FileOpened(result) => {
                if let Ok((path, contents)) = result {
                    self.file = Some(path);
                    // self.content = todo!();
                };
                Command::none()
            }
            Message::Correction => {
                self.last_score =
                    self.content[self.current.unwrap()].correct(&self.entry.trim().into());
                self.total_score.0 += self.last_score;
                self.state = State::Correcting;
                Command::none()
            }
            Message::Next => {
                self.entry = String::new();
                match self.current {
                    Some(nb) => {
                        self.current = if nb + 1 == self.content.len() {
                            self.state = State::NotRunning;
                            None
                        } else {
                            self.state = State::WaitUserAnswer;
                            Some(nb + 1)
                        }
                    }
                    None => (),
                }
                Command::none()
            }
            Message::None => Command::none(),
            Message::OpenFile => Command::perform(pick_file(), Message::FileOpened),
            Message::OpenSettings => Command::none(),
            Message::Start => {
                self.state = State::WaitUserAnswer;
                Command::none()
            }
            Message::ThemeSelected => {
                self.dark_theme = !self.dark_theme;
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let max_len = *(self
            .langs
            .clone()
            .map(|lang| format!("{}", lang).len())
            .iter())
        .max()
        .unwrap_or(&15) as u16
            * 20;

        let head_one = text(&self.langs[0]).width(max_len);
        let head_two = text(&self.langs[1]).width(max_len);

        let mut first_row = row![head_one].padding(2);
        if let State::WaitUserAnswer = self.state {
            first_row = first_row.push(
                text_input("Write your answer", &self.entry)
                    .on_input(Message::TextInputChanged)
                    .on_submit(match self.state {
                        State::WaitUserAnswer => Message::Correction,
                        State::Correcting => Message::Next,
                        State::Starting => Message::Start,
                        _ => Message::None,
                    }),
            );
        }
        if let State::Correcting = self.state {
            first_row = first_row
                .push(text(&self.entry))
                .push_maybe(if self.current.is_some() && !self.entry.is_empty() {
                    Some(Space::new(10, 0))
                } else {
                    None
                })
                .push_maybe(match &self.current {
                    Some(nb) => Some(text(&self.content[*nb].get(0))),
                    None => None,
                });
        }
        let known = text(match self.current {
            Some(nb) => self.content[nb].get(1),
            None => "".into(),
        });

        let score = text(&format!(
            "{} / 1\n{} / {} ({})",
            self.last_score,
            self.total_score.0,
            self.current.unwrap_or(0) + 1,
            self.total_score.1,
        ));
        let next_button = button(match self.state {
            State::Starting => "Begin",
            State::Correcting => "Next",
            State::WaitUserAnswer => "Correct",
            _ => "",
        })
        .on_press(match self.state {
            State::Starting => Message::Start,
            State::Correcting => Message::Next,
            State::WaitUserAnswer => Message::Correction,
            _ => Message::None,
        });

        let open = button("Open").on_press(Message::OpenFile);
        let theme = toggler(Some("Theme".into()), self.dark_theme, |_| Message::ThemeSelected);
        let settings = button("Settings").on_press(Message::OpenSettings);

        let header = row![open, horizontal_space(), theme, settings].padding(5);
        container(column![
            header,
            first_row,
            Space::new(Length::Fill, 10),
            row![head_two, known].padding(2),
            Space::new(Length::Fill, 10),
            row!(
                horizontal_space(),
                text("Score: ")
                    .vertical_alignment(Vertical::Center)
                    .horizontal_alignment(Horizontal::Right),
                score.horizontal_alignment(Horizontal::Right),
                // Space::new(10, Length::Fill),
                next_button,
            )
            .spacing(10)
            .padding(10),
        ])
        .padding(10)
        .into()
    }

    fn theme(&self) -> Theme {
        if self.dark_theme {
            Theme::Dark
        } else {
            Theme::Light
        }
    }
}

// #[cfg(not(target_family = "wasm"))]
async fn pick_file() -> Result<(PathBuf, Arc<String>), Error> {
    let opt_handle = rfd::AsyncFileDialog::new()
        .set_title("Choose a text file...")
        .pick_file()
        .await;
    // load_file(handle.path().to_owned()).await
    match opt_handle {
        Some(handle) => {
            let path = handle.path();
            match async_std::fs::read_to_string(path).await {
                Ok(raw) => Ok((path.into(), Arc::new(raw))),
                Err(_) => Err(Error::IoError),
            }
        }
        None => Err(Error::DialogClosed),
    }
}
