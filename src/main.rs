use iced::{
    executor,
    widget::{button, column, container, horizontal_space, row, space::Space, text, text_input},
    Application, Command, Element, Length, Settings, Theme,
};
use rand::{seq::SliceRandom, thread_rng};
use std::{io, path::PathBuf, sync::Arc, vec};

mod word;
use word::*;

fn main() -> iced::Result {
    App::run(Settings::default())
}

struct App {
    content: Vec<Entry>,
    current: Option<usize>,
    entry: String,
    error: Option<Error>,
    file: Option<PathBuf>,
}

#[derive(Debug, Clone)]
enum Message {
    TextInputChanged(String),
    FileOpened(Result<(PathBuf, Arc<String>), Error>),
}

#[derive(Debug, Clone)]
enum Error {
    IoError(io::ErrorKind),
    DialogClosed,
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
                    content.shuffle(&mut thread_rng());
                    content
                },
                current: Some(0),
                entry: String::new(),
                error: None,
                file: None,
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
                todo!();
            }
            Message::FileOpened(result) => {
                if let Ok((path, contents)) = result {
                    self.file = Some(path);
                    self.content = temporary(); // TODO!
                };
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        // let controls = row![button("Open").on_press(Message::Open)];
        // let input = text_editor(&self.content)
        //     .height(Length::Fill)
        //     .on_action(Message::Edit);
        // let file_path = match self.path.as_deref().and_then(Path::to_str) {
        //     Some(path) => text(path).size(14),
        //     None => text(""),
        // };

        let head_one = text("Français");
        let head_two = text("Anglais");

        let input = text_input("Write your answer", &self.entry);
        let known = text((&self.content[self.current.unwrap()]).get(0));

        container(row![
            column![head_one.height(2.), head_two,],
            Space::with_width(Length::Fixed(20.)),
            column![input.on_input(Message::TextInputChanged), known,],
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

async fn pick_file() -> Result<(PathBuf, Arc<String>), Error> {
    let handle = rfd::AsyncFileDialog::new()
        .set_title("Choose a text file...")
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;
    load_file(handle.path().to_owned()).await
}

fn temporary() -> Vec<Entry> {
    vec![
        Entry("yes".into(), "oui".into(), GramClass::Noun),
        Entry("der Gast".into(), "l'invité".into(), GramClass::Noun),
        Entry("die Arbeit".into(), "le travail".into(), GramClass::Noun),
        Entry("die Heimat".into(), "la patrie".into(), GramClass::Noun),
        Entry("die Lösung".into(), "la solution".into(), GramClass::Noun),
        Entry("die Ankunft".into(), "l'arrivée".into(), GramClass::Noun),
    ]
}
