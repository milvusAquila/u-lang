use iced::{
    advanced::Widget,
    widget::{slider, toggler},
};
use iced_aw::menu;

use crate::{App, Message, Theme};

impl App {
    pub fn view_settings(&self) -> menu::Menu<Message, Theme, iced::Renderer> {
        let menu_tpl = |items| {
            menu::Menu::new(items)
                .max_width(180.0)
                .offset(5.0)
                .spacing(5.0)
        };

        let theme: iced::widget::Toggler<Message> =
            toggler(Some("Theme".into()), self.dark_theme, |_| {
                Message::ThemeSelected
            })
            .size(self.font_size);
        let text_size = slider(10.0..=50., self.font_size.0, Message::TextFontChanged);

        let size = Widget::size(&theme).width;
        menu_tpl(iced_aw::menu_items!((theme)(text_size))).width(size)
    }
}
