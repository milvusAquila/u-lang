use iced::{
    alignment::Horizontal,
    executor,
    widget::{button, column, container, row, text, text_input},
    Application, Command, Element, Theme,
};
use rand::{seq::SliceRandom, thread_rng};
use std::{io, path::PathBuf, sync::Arc, vec};

mod word;
use word::*;

pub struct App {
    content: Vec<Entry>,
    current: Option<usize>,
    entry: String,
    error: Option<Error>,
    file: Option<PathBuf>,
    langs: [String; 2],
    state: State,
    last_score: f32,
    total_score: (f32, u16),
}

#[derive(Debug, Clone)]
pub enum Message {
    TextInputChanged(String),
    FileOpened(Result<(PathBuf, Arc<String>), Error>),
    Correction,
    Next,
    None,
}

#[derive(Debug, Clone)]
enum Error {
    IoError(io::ErrorKind),
    DialogClosed,
}

#[derive(Debug)]
enum State {
    Correcting,
    WaitUserAnswer,
    Finish,
}

impl Application for App {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flag: Self::Flags) -> (Self, Command<Message>) {
        (
            Self {
                content: {
                    let mut content = temporary();
                    println!("{:?}", &content);
                    content.shuffle(&mut thread_rng());
                    println!("{:?}", &content);
                    content
                },
                current: Some(0),
                entry: String::new(),
                error: None,
                file: None,
                langs: ["German".into(), "French".into()],
                state: State::WaitUserAnswer,
                last_score: 0.,
                total_score: (0., 0),
            },
            Command::perform(load_file(default_file()), Message::FileOpened),
        )
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
                    self.content = temporary(); // TODO!
                };
                Command::none()
            }
            Message::Correction => {
                self.last_score = self.content[self.current.unwrap()].correct(&self.entry);
                self.total_score.0 += self.last_score;
                self.state = State::Correcting;
                Command::none()
            }
            Message::Next => {
                match self.current {
                    Some(nb) => {
                        self.current = if nb + 1 == self.content.len() {
                            self.state = State::Finish;
                            None
                        } else {
                            self.entry = String::new();
                            Some(nb + 1)
                        }
                    }
                    None => (),
                }
                Command::none()
            }
            Message::None => Command::none(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let max_len = *(self.langs.clone().map(|lang| lang.len()).iter())
            .max()
            .unwrap_or(&15);
        let max_len = max_len as u16 * 20;
        let head_one = text(&self.langs[0]).width(max_len);
        let head_two = text(&self.langs[1]).width(max_len);

        let input = text_input("Write your answer", &self.entry);
        let known = text((&self.content[self.current.unwrap()]).get(0));

        let last_score = text(&format!("Last score: {}", self.last_score));
        let global_score = text(&format!(
            "Score: {} / {} ({})",
            self.total_score.0,
            self.current.unwrap_or(0),
            self.total_score.1,
        ));

        container(column![
            row![
                head_one,
                input
                    .on_input(Message::TextInputChanged)
                    .on_submit(match self.state {
                        State::WaitUserAnswer => Message::Correction,
                        State::Correcting => Message::Next,
                        State::Finish => Message::None,
                    })
            ],
            row![head_two, known],
            row!(
                last_score,
                global_score.horizontal_alignment(Horizontal::Right)
            ),
        ])
        .padding(10)
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

fn default_file() -> PathBuf {
    PathBuf::from(format!(
        "{}/assets/default.json",
        env!("CARGO_MANIFEST_DIR")
    ))
}

async fn load_file(path: PathBuf) -> Result<(PathBuf, Arc<String>), Error> {
    let contents = tokio::fs::read_to_string(&path)
        .await
        .map(Arc::new)
        .map_err(|error| Error::IoError(error.kind()))?;
    Ok((path, contents))
}

async fn _pick_file() -> Result<(PathBuf, Arc<String>), Error> {
    let handle = rfd::AsyncFileDialog::new()
        .set_title("Choose a text file...")
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;
    load_file(handle.path().to_owned()).await
}

fn temporary() -> Vec<Entry> {
    vec![
        Entry("yes".into(), "oui".into(), GramClass::Adverb),
        Entry("der Gast".into(), "l'invité".into(), GramClass::Noun),
        Entry("die Arbeit".into(), "le travail".into(), GramClass::Noun),
        Entry("die Heimat".into(), "la patrie".into(), GramClass::Noun),
        Entry("die Lösung".into(), "la solution".into(), GramClass::Noun),
        Entry("die Ankunft".into(), "l'arrivée".into(), GramClass::Noun),
    ]
}
