use iced::{widget::button, Theme};

#[derive(Debug, Clone)]
pub struct Header{
    theme: Theme
}

impl button::StyleSheet for Header {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(self.theme.palette().background)),
            text_color: self.theme.palette().text,
            ..Default::default()
        }
    }
}

impl<'a> From<&Theme> for Header {
    fn from(value: &Theme) -> Self {
        Header { theme: value.clone() }
    }
}
