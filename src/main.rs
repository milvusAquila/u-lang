use iced::{
    executor,
    widget::{button, column, container, horizontal_space, row, text, text_input},
    Application, Command, Element, Settings, Theme,
};
use std::{io, path::PathBuf, sync::Arc};

fn main() -> iced::Result {
    App::run(Settings::default())
}

struct App {
    content: Vec<String>,
    current: Option<u8>,
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
                content: Vec::new(),
                current: None,
                error: None,
                file: None,
            },
            Command::perform(
                load_file(default_file()),
                Message::FileOpened,
            ),
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
                    self.content = todo!();
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

        let input_one = text_input("Write your answer", &self.content[0]);
        let input_two = text_input("Write your answer", &self.content[0]);

        container(column![
            row![head_one, input_one.on_input(Message::TextInputChanged)],
            row![head_two, input_two.on_input(Message::TextInputChanged)],
        ])
        .padding(10)
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

fn default_file() -> PathBuf {
    PathBuf::from(format!("{}/assets/default.json", env!("CARGO_MANIFEST_DIR")))
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
