#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use iced::{
    alignment,
    keyboard::{self, Key},
    theme,
    widget::{button, column, container, progress_bar, row, space::Space, text, text_input},
    Alignment, Application, Command, Element, Length, Pixels, Size, Theme,
};
use iced_aw::menu::{self, Item};
use iced_aw::{grid, grid_row};
use rand::{seq::SliceRandom, thread_rng};
use std::{path::PathBuf, sync::Arc, vec};
use style::style_text;

use grammar::*;
mod settings;
mod style;

fn main() -> iced::Result {
    App::run(iced::Settings {
        window: iced::window::Settings {
            size: Size::new(450., 200.),
            min_size: Some(Size::new(450., 200.)),
            ..Default::default()
        },
        ..Default::default()
    })
}

#[derive(Debug)]
struct App {
    debug_layout: bool,
    content: Vec<Entry>,
    current: Option<usize>,
    entry: String,
    error: Option<Error>,
    file: Option<PathBuf>,
    langs: [Lang; 2],
    state: State,
    last_score: f32,
    score: f32,
    length: usize,
    dark_theme: bool,
    font_size: Pixels,
    spacing: f32,
}

impl App {
    fn init(&mut self, mut content: Vec<Entry>) {
        self.entry = String::new();
        self.current = Some(0);
        content.shuffle(&mut thread_rng());
        self.content = content;
        self.score = 0.0;
        self.length = self.content.len();
        self.last_score = 0.;
        self.state = State::WaitUserAnswer;
    }
    fn correct(&mut self) {
        self.last_score = self.content[self.current.unwrap()].correct(
            &self.entry.trim().into(),
            0,
            &self.langs[0],
        );
        self.score += self.last_score;
        self.state = State::Correcting;
    }
    fn next(&mut self) {
        self.entry = String::new();
        match self.current {
            Some(nb) => {
                self.current = if nb + 1 == self.content.len() {
                    self.state = State::End;
                    Some(nb)
                } else {
                    self.state = State::WaitUserAnswer;
                    Some(nb + 1)
                }
            }
            None => (),
        }
    }
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
            debug_layout: false,
            score: 0.0,
            length: default_content.len(),
            content: default_content,
            current: Some(0),
            entry: String::new(),
            error: None,
            file: None,
            langs: ["English".into(), "French".into()],
            state: State::WaitUserAnswer,
            last_score: 0.,
            dark_theme: true,
            font_size: Pixels(16.),
            spacing: 5.0,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    DebugToggle,
    TextInputChanged(String),
    OpenFile,
    FileOpened(Result<(PathBuf, Arc<([Lang; 2], Vec<Entry>)>), Error>),
    Correction,
    Next,
    // None,
    Start,
    Enter,
    ThemeSelected,
    TextFontChanged(f32),
    SpacingChanged(f32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Error {
    IoError,
    DialogClosed,
    ParseError,
}

#[derive(Debug)]
enum State {
    Correcting,
    WaitUserAnswer,
    End,
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
        match &self.file {
            Some(path) => format!("{} — ULang ", path.to_str().unwrap_or("")),
            None => String::from("ULang"),
        }
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        keyboard::on_key_press(|key, modifiers| match key.as_ref() {
            Key::Character("o") if modifiers.command() => Some(Message::OpenFile), // Ctrl + o
            Key::Named(keyboard::key::Named::Enter) => Some(Message::Enter),       // Enter
            _ => None,
        })
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::DebugToggle => {
                self.debug_layout = !self.debug_layout;
                Command::none()
            }
            Message::TextInputChanged(value) => {
                self.entry = value;
                Command::none()
            }
            Message::OpenFile => Command::perform(pick_file(), Message::FileOpened),
            Message::FileOpened(result) => {
                match result {
                    Ok((path, content)) => {
                        self.langs = content.0.clone();
                        self.init(content.1.clone());
                        self.file = Some(path);
                        self.error = None;
                    }
                    Err(Error::DialogClosed) => (),
                    Err(err) => self.error = Some(err),
                }
                Command::none()
            }
            Message::Enter => {
                match self.state {
                    State::WaitUserAnswer => self.correct(),
                    State::Correcting => self.next(),
                    _ => (),
                }
                Command::none()
            }
            Message::Correction => {
                self.correct();
                Command::none()
            }
            Message::Next => {
                self.next();
                Command::none()
            }
            // Message::None => Command::none(),
            Message::Start => {
                if let Some(_) = self.file {
                    self.init(self.content.clone());
                } else {
                    self.init(App::default().content);
                }
                self.state = State::WaitUserAnswer;
                Command::none()
            }
            Message::ThemeSelected => {
                self.dark_theme = !self.dark_theme;
                Command::none()
            }
            Message::TextFontChanged(new_size) => {
                self.font_size.0 = new_size;
                Command::none()
            }
            Message::SpacingChanged(new_spacing) => {
                self.spacing = new_spacing;
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        // Declarating widgets
        let lang_one = style_text(text(&self.langs[0]), self.font_size);
        let lang_two = style_text(text(&self.langs[1]), self.font_size);

        let known = style_text(
            text(match self.current {
                Some(nb) => match self.state {
                    State::End => "".into(),
                    _ => self.content[nb].get(1),
                },
                None => "".into(),
            }),
            self.font_size,
        );

        let next_button = button(
            text(match self.state {
                State::Correcting => "Next",
                State::WaitUserAnswer => "Correct",
                State::End => "Restart",
            })
            .size(self.font_size)
            .width(self.font_size * 4.0)
            .horizontal_alignment(alignment::Horizontal::Center),
        )
        .on_press(match self.state {
            State::Correcting => Message::Next,
            State::WaitUserAnswer => Message::Correction,
            State::End => Message::Start,
        });

        let open = button(text("Open").size(self.font_size))
            .on_press(Message::OpenFile)
            .style(theme::Button::Custom(Box::new(style::Header::from(
                &self.theme(),
            ))));

        let menu_tpl = |items| {
            menu::Menu::new(items)
                .max_width(180.0)
                .offset(5.0)
                .spacing(5.0)
        };

        let error_log = text(match &self.error {
            Some(err) => format!("{:?}: invalid file", err),
            None => "".to_string(),
        })
        .size(self.font_size);

        // Header
        #[rustfmt::skip]
        let header = iced_aw::menu_bar!(
            (button(text("File").size(self.font_size))
                .style(theme::Button::Custom(Box::new(style::Header::from(&self.theme())))),
                // see in src/style.rs
            {
                menu_tpl(iced_aw::menu_items!((open))).width(Length::Shrink)
            })
            (button(text("Settings").size(self.font_size))
                .style(theme::Button::Custom(Box::new(style::Header::from(&self.theme())))),
            {
                self.view_settings() // see in src/settings.rs
            })
        );

        // Main
        let mut variable = row![]
            .width(self.font_size.0 * 20.0)
            .align_items(Alignment::Center);
        match self.state {
            State::WaitUserAnswer => {
                variable = variable.push(
                    text_input("Write your answer", &self.entry)
                        .size(self.font_size)
                        .on_input(Message::TextInputChanged)
                        .on_submit(Message::Correction),
                );
            }
            State::Correcting => {
                let nb = self
                    .current
                    .expect("ERROR: current index in the data base is set to None");
                if self.entry.trim().is_empty() {
                    variable = variable.push(style_text(
                        text(&self.content[nb].get(0)).style(style::TextColor::Red),
                        self.font_size,
                    ));
                } else if self.last_score != 1.0 {
                    variable = variable
                        .push(style_text(
                            text(&self.entry).style(style::TextColor::Red),
                            self.font_size,
                        ))
                        .push(Space::with_width(Length::Fixed(10.0)))
                        .push(style_text(
                            text(&self.content[nb].get(0)).style(style::TextColor::Green),
                            self.font_size,
                        ));
                } else {
                    variable = variable.push(style_text(
                        text(&self.content[nb].get(0)).style(style::TextColor::Green),
                        self.font_size,
                    ));
                }
                variable = variable
                    .push(Space::with_width(Length::Fixed(10.0)))
                    .push(text(self.last_score).size(self.font_size));
            }
            _ => (),
        }

        // Score
        let current = self.current.unwrap_or(0);
        let max = self.length - 1;
        let score = text(format!(
            "{} / {}",
            self.score,
            self.current.unwrap_or(0) + 1
        ))
        .size(self.font_size);
        let advancement = progress_bar(0.0..=max as f32, current as f32).height(7.0);
        let advancement_text = text(format!("{} %", current * 100 / max)).size(self.font_size);

        // Final
        let grid = column![
            row![header],
            grid!(grid_row!(lang_one, variable), grid_row!(lang_two, known)).spacing(self.spacing),
            row![Space::with_width(Length::Fill), score, next_button,]
                .spacing(self.spacing / 2.0)
                .align_items(Alignment::Center),
            row![advancement, advancement_text,]
                .spacing(self.spacing)
                .align_items(Alignment::Center),
            Space::with_height(Length::Fill),
        ]
        .push_maybe(match &self.error {
            Some(_) => Some(error_log),
            None => None,
        })
        .spacing(self.spacing);
        let mut contents = Element::from(grid);
        if self.debug_layout {
            contents = contents.explain(iced::Color::WHITE);
        }
        let ctnr_padding = if self.spacing < 10.0 {
            self.spacing
        } else {
            10.0
        };
        container(contents)
            .padding([0.0, ctnr_padding, ctnr_padding, ctnr_padding])
            .width(Length::Fill)
            .height(Length::Shrink)
            .center_x()
            .align_y(alignment::Vertical::Center)
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

#[cfg(not(target_family = "wasm"))]
async fn pick_file() -> Result<(PathBuf, Arc<([Lang; 2], Vec<Entry>)>), Error> {
    let opt_handle = rfd::AsyncFileDialog::new()
        .set_title("Choose a json file...")
        .add_filter("Json (*.json)", &["json"])
        .add_filter("All files (*.*)", &["*"])
        .pick_file()
        .await;
    match opt_handle {
        Some(handle) => {
            let path = handle.path();
            match async_std::fs::read_to_string(path).await {
                Ok(raw) => match parse(&raw) {
                    Ok(data) => Ok((path.into(), Arc::new(data))),
                    Err(_) => Err(Error::ParseError),
                },
                Err(_) => Err(Error::IoError),
            }
        }
        None => Err(Error::DialogClosed),
    }
}
