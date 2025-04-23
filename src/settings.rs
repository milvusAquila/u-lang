use iced::widget::{slider, text, toggler};
use iced_aw::menu;

use crate::{App, Message, Theme};

impl App {
    pub fn view_settings(&self) -> menu::Menu<Message, Theme, iced::Renderer> {
        let menu_tpl = |items| {
            menu::Menu::new(items)
                .max_width(11.0 * self.font_size.0)
                .offset(5.0)
                .spacing(5.0)
        };

        let theme: iced::widget::Toggler<Message> = toggler(self.dark_theme)
            .label("Theme")
            .on_toggle(|_| Message::ThemeSelected)
            .size(self.font_size)
            .text_size(self.font_size);

        let font_size_header = text("Text size").size(self.font_size);
        let font_size_slidder = slider(10.0..=50.0, self.font_size.0, Message::TextFontSizeChanged);

        let spacing_header = text("Spacing").size(self.font_size);
        let spacing_slider = slider(0.0..=20.0, self.spacing, Message::SpacingChanged);

        let debug_layout = toggler(self.debug_layout)
            .label("Debug layout")
            .on_toggle(|_| Message::DebugToggle)
            .size(self.font_size)
            .text_size(self.font_size);

        #[rustfmt::skip]
        let settings = menu_tpl(iced_aw::menu_items!(
            (theme)
            (font_size_header)
            (font_size_slidder)
            (spacing_header)
            (spacing_slider)
            (debug_layout)
        ));
        settings
    }
}
