use crate::{style, App, Message};
use iced::{
    widget::{button, column, text},
    Element, Length,
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
            (button(text("ings").size(self.font_size))
                .style(style::header_button),
            {
                self.view_settings() // see in src/settings.rs
            })
        );

        Element::from(column![header])
    }
}
