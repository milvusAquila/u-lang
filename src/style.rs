use iced::{
    advanced::widget::text, alignment, color, widget::button, Length, Pixels, Renderer, Theme,
};

#[derive(Debug, Clone)]
pub struct Header {
    theme: Theme,
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
        Header {
            theme: value.clone(),
        }
    }
}

pub fn style_text(
    text: text::Text<Theme, Renderer>,
    font_size: Pixels,
) -> text::Text<Theme, Renderer> {
    text.size(font_size)
        .height(Length::Fixed(font_size.0 * 1.3 + 10.0))
        // 1.3 is the default value for LineHeight // 2 * 5.0 is the padding
        .vertical_alignment(alignment::Vertical::Center)
}

#[derive(Clone)]
pub enum TextColor {
    Red,
    Green,
}

impl text::StyleSheet for TextColor {
    type Style = Theme;

    fn appearance(&self, _style: Self::Style) -> text::Appearance {
        text::Appearance {
            color: Some(color!(match self {
                TextColor::Red => 0xff0000,
                TextColor::Green => 0x00ff00,
            })),
        }
    }
}

impl From<TextColor> for iced::theme::Text {
    fn from(value: TextColor) -> Self {
        iced::theme::Text::Color(match value {
            TextColor::Red => color!(0xff0000),
            TextColor::Green => color!(0x00ff00),
        })
    }
}
