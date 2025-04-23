use crate::{
    style::{self, header_button},
    App, Message,
};
use iced::{
    widget::{button, column, container, row, scrollable, text, text_input},
    Alignment, Element, Length,
};
use iced_aw::menu::{self, Item};

impl App {
    pub fn screen_editor(&self) -> Element<'_, Message> {
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

        let editor = button(text("Close").size(self.font_size))
            .on_press(Message::EditorClosed(()))
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
        let mut main = column![].width(Length::Fill);
        for (i, value) in self.content.iter().enumerate() {
            main = main.push(
                row![
                    if self.current == Some(i) {
                        container(
                            text_input(value.get(0).as_str(), &value.get(0))
                                .size(self.font_size)
                                .id(self.input_id.clone())
                                .on_input(Message::TextInputChanged),
                        )
                        .width(Length::FillPortion(1))
                    } else {
                        container(
                            button(text(format!("{}", value.get(0))).size(self.font_size))
                                .on_press(Message::EditText(i))
                                .width(Length::Fill)
                                .style(header_button),
                        )
                        .width(Length::FillPortion(1))
                    },
                    text(format!("{}", value.get(1)))
                        .size(self.font_size)
                        .align_y(Alignment::Center)
                        .width(Length::FillPortion(1))
                ]
                .padding(self.spacing),
            );
        }

        Element::from(column![header, scrollable(main).width(Length::Fill)])
    }
}
