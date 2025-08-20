#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use iced::{
    alignment,
    keyboard::{self, Key},
    widget::{button, column, container, progress_bar, row, text, text_input, Space},
    Alignment, Element, Length, Pixels, Size, Task, Theme,
};
use iced_aw::menu::{self, Item};
use iced_aw::{grid, grid_row};
use rand::{seq::SliceRandom, thread_rng};
use std::{path::PathBuf, sync::Arc, vec};
use style::style_text;

mod grammar;
use grammar::{parse, Entry, GramClass, Lang};
mod editor;
mod settings;
mod style;

fn main() -> iced::Result {
    iced::application(App::title, App::update, App::view)
        .window(iced::window::Settings {
            size: Size::new(700., 400.),
            min_size: Some(Size::new(700., 400.)),
            ..Default::default()
        })
        .subscription(App::subscription)
        .theme(App::theme)
        .run()
}

#[derive(Debug)]
struct App {
    debug_layout: bool,
    screen: Screen,
    content: Vec<Entry>,
    order: Vec<usize>,
    current: Option<usize>,
    entry: String,
    error: Option<Error>,
    file: Option<PathBuf>,
    langs: [Lang; 2],
    score: (f32, f32),
    length: usize,
    dark_theme: bool,
    font_size: Pixels,
    spacing: f32,
    input_id: text_input::Id,
}

impl App {
    fn init(&mut self, content: Vec<Entry>) {
        self.entry = String::new();
        self.current = Some(0);
        self.content = content;
        self.score = (0.0, 0.0);
        self.length = self.content.len();
        let mut order = (0..self.length).collect::<Vec<usize>>();
        order.shuffle(&mut thread_rng());
        self.order = order;
        self.screen = Screen::Main(State::WaitUserAnswer);
    }

    fn correct(&mut self) {
        let index = self.order[self.current.unwrap()];
        self.score.0 = self.content[index].correct(&self.entry.trim().into(), 0, &self.langs[0]);
        self.score.1 += self.score.0;
        self.screen = Screen::Main(State::Correcting);
    }

    fn next(&mut self) {
        self.entry = String::new();
        match self.current {
            Some(nb) => {
                self.current = if nb + 1 == self.content.len() {
                    self.screen = Screen::Main(State::End);
                    Some(nb)
                } else {
                    self.screen = Screen::Main(State::WaitUserAnswer);
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
            screen: Screen::default(),
            score: (0.0, 0.0),
            length: default_content.len(),
            order: (0..default_content.len()).collect(),
            content: default_content,
            current: Some(0),
            entry: String::new(),
            error: None,
            file: None,
            langs: ["English".into(), "French".into()],
            dark_theme: true,
            font_size: Pixels(16.0),
            spacing: 5.0,
            input_id: text_input::Id::unique(),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    DebugToggle,
    TextInputChanged(String),
    OpenFile,
    FileOpened(Result<(PathBuf, Arc<([Lang; 2], Vec<Entry>)>), Error>),
    OpenEditor,
    EditText(usize),
    EditorClosed(()),
    Correction,
    Next,
    Start,
    Enter,
    ThemeSelected,
    TextFontSizeChanged(f32),
    SpacingChanged(f32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Screen {
    Main(State),
    Editor(Vec<[text_input::Id; 2]>),
}

impl Default for Screen {
    fn default() -> Self {
        Self::Main(State::default())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Error {
    IoError,
    DialogClosed,
    ParseError,
}

#[derive(Debug, Default, Eq, PartialEq, Clone, Copy)]
enum State {
    Correcting,
    #[default]
    WaitUserAnswer,
    End,
}

impl App {
    fn title(&self) -> String {
        match &self.file {
            Some(path) => format!("{} — ULang ", path.to_str().unwrap_or("")),
            None => String::from("ULang"),
        }
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        keyboard::on_key_press(|key, modifiers| match key.as_ref() {
            Key::Character("o") if modifiers.command() => Some(Message::OpenFile), // Ctrl + o
            Key::Character("e") if modifiers.command() => Some(Message::OpenEditor), // Ctrl + e
            Key::Named(keyboard::key::Named::Enter) => Some(Message::Enter),       // Enter
            _ => None,
        })
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::DebugToggle => {
                self.debug_layout = !self.debug_layout;
                Task::none()
            }
            Message::TextInputChanged(value) => {
                let index = self.order[self.current.unwrap()];
                match self.screen {
                    Screen::Main(_) => self.entry = value,
                    Screen::Editor(_) => self.content[index].0 = (&value).into(),
                }
                Task::none()
            }
            Message::OpenFile => Task::perform(pick_file(), Message::FileOpened),
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
                Task::none()
            }
            Message::OpenEditor => {
                self.current = None;
                self.screen = Screen::Editor(
                    self.content
                        .iter()
                        .map(|_| [text_input::Id::unique(), text_input::Id::unique()])
                        .collect::<Vec<_>>(),
                );
                Task::none()
            }
            Message::EditText(index) => {
                self.current = Some(index);
                iced::widget::text_input::focus(self.input_id.clone())
            }
            Message::EditorClosed(()) => {
                self.screen = Screen::Main(State::WaitUserAnswer);
                self.current = Some(0);
                Task::none()
            }
            Message::Enter => {
                if let Screen::Main(state) = self.screen {
                    match state {
                        State::WaitUserAnswer => self.correct(),
                        State::Correcting => self.next(),
                        _ => (),
                    }
                }
                text_input::focus::<Message>(self.input_id.clone())
            }
            Message::Correction => {
                self.correct();
                Task::none()
            }
            Message::Next => {
                self.next();
                text_input::focus::<Message>(self.input_id.clone())
            }
            Message::Start => {
                if let Some(_) = self.file {
                    self.init(self.content.clone());
                } else {
                    self.init(App::default().content);
                }
                self.screen = Screen::Main(State::WaitUserAnswer);
                text_input::focus::<Message>(self.input_id.clone())
            }
            Message::ThemeSelected => {
                self.dark_theme = !self.dark_theme;
                Task::none()
            }
            Message::TextFontSizeChanged(new_size) => {
                self.font_size.0 = new_size;
                Task::none()
            }
            Message::SpacingChanged(new_spacing) => {
                self.spacing = new_spacing;
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let mut screen = match self.screen {
            Screen::Main(_) => self.screen_main(),
            Screen::Editor(_) => self.screen_editor(), // see in src/editor.rs
        };
        if self.debug_layout {
            screen = screen.explain(iced::Color::WHITE);
        }
        container(screen)
            .padding(
                iced::Padding::from(if self.spacing < 10.0 {
                    self.spacing
                } else {
                    10.0
                })
                .top(0.0),
            )
            .center(Length::Shrink)
            .into()
    }

    fn theme(&self) -> Theme {
        if self.dark_theme {
            Theme::Dark
        } else {
            Theme::Light
        }
    }

    fn screen_main(&self) -> Element<'_, Message> {
        /*
         *   |File|Settings|                         -> Header
         *   ---------------
         *   |English| {answer / correction} |       -|
         *   |French | {known element}       |       -> Main
         *   ---------------
         *                  {score} {continue}       -|
         *          {------progress bar------}       -> Score
         */
        let state = match self.screen {
            Screen::Main(state) => state,
            _ => panic!(),
        };
        // Header
        let menu_tpl = |items| {
            menu::Menu::new(items)
                .max_width(180.0)
                .offset(5.0)
                .spacing(5.0)
        };

        let open = button(text("Open").size(self.font_size))
            .on_press(Message::OpenFile)
            .style(style::header_button);

        let editor = button(text("Edit").size(self.font_size))
            .on_press(Message::OpenEditor)
            .style(style::header_button);

        #[rustfmt::skip]
        let header = iced_aw::menu_bar!(
            (button(text("File").size(self.font_size))
                .style(style::header_button), // see in src/style.rs
            {
                menu_tpl(iced_aw::menu_items!(
                    (open)
                    (editor)
                )).width(Length::Shrink)
            })
            (button(text("Settings").size(self.font_size))
                .style(style::header_button),
            {
                self.view_settings() // see in src/settings.rs
            })
        );

        // Main
        let lang_one = style_text(text(self.langs[0].to_string()), self.font_size);
        let lang_two = style_text(text(self.langs[1].to_string()), self.font_size);

        let known = style_text(
            text(match self.current {
                Some(nb) if state != State::End => self.content[self.order[nb]].get(1),
                _ => "".into(),
            }),
            self.font_size,
        );

        let next_button = button(
            text(match state {
                State::Correcting => "Next",
                State::WaitUserAnswer => "Correct",
                State::End => "Restart",
            })
            .size(self.font_size)
            .width(self.font_size * 4.0)
            .align_x(alignment::Horizontal::Center),
        )
        .on_press(match state {
            State::Correcting => Message::Next,
            State::WaitUserAnswer => Message::Correction,
            State::End => Message::Start,
        });

        let error_log = text(match &self.error {
            Some(err) => format!("{:?}: invalid file", err),
            None => "".to_string(),
        })
        .size(self.font_size);

        // Main
        let mut variable = row![].padding(self.spacing).align_y(Alignment::Center);
        match state {
            State::WaitUserAnswer => {
                variable = variable.push({
                    text_input("Write your answer", &self.entry)
                        .id(self.input_id.clone())
                        .size(self.font_size)
                        .on_input(Message::TextInputChanged)
                        .on_submit(Message::Correction)
                });
            }
            State::Correcting => {
                let nb = self
                    .current
                    .expect("ERROR: current index in the data base is set to None");
                if self.entry.trim().is_empty() {
                    variable = variable.push(style_text(
                        text(self.content[self.order[nb]].get(0)).color(style::TextColor::Red),
                        self.font_size,
                    ));
                } else if self.score.0 != 1.0 {
                    variable = variable
                        .push(style_text(
                            text(&self.entry).color(style::TextColor::Red),
                            self.font_size,
                        ))
                        .push(Space::with_width(Length::Fixed(10.0)))
                        .push(style_text(
                            text(self.content[self.order[nb]].get(0))
                                .color(style::TextColor::Green),
                            self.font_size,
                        ));
                } else {
                    variable = variable.push(style_text(
                        text(self.content[self.order[nb]].get(0)).color(style::TextColor::Green),
                        self.font_size,
                    ));
                }
                variable = variable
                    .push(Space::with_width(Length::Fixed(10.0)))
                    .push(text(self.score.0).size(self.font_size));
            }
            _ => (),
        }

        // Score
        let current = self.current.unwrap_or(0);
        let max = self.length - 1;
        let score = text(format!(
            "{} / {}{}",
            self.score.1,
            current + 1,
            if state == State::End {
                format!(" ({:.2} / 20)", self.score.1 * 20.0 / (current + 1) as f32)
            } else {
                "".to_string()
            }
        ))
        .size(self.font_size);
        let advancement = progress_bar(0.0..=max as f32, current as f32).height(7.0);
        let advancement_text = text(format!("{} %", current * 100 / max)).size(self.font_size);

        // Final
        let grid = column![
            // Header
            header,
            // Main
            grid!(grid_row!(lang_one, variable), grid_row!(lang_two, known))
                .spacing(self.spacing)
                .column_widths(&[Length::Shrink, Length::Fill])
                .width(Length::Fill),
            // Score
            row![Space::with_width(Length::Fill), score, next_button,]
                .spacing(self.spacing / 2.0)
                .align_y(Alignment::Center),
            row![advancement, advancement_text,]
                .spacing(self.spacing)
                .align_y(Alignment::Center),
            Space::with_height(Length::Fill),
        ]
        .push_maybe(match &self.error {
            Some(_) => Some(error_log),
            None => None,
        })
        .spacing(self.spacing);
        Element::from(grid)
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
