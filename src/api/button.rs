
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MessageButton {
    Pressed,
}

struct Button<'a, Message>(iced::widget::Button<'a, Message>);

impl<'a, Message> Button<'a, Message> {
    pub fn new(content: iced::Element<'a, Message>) -> Self {
        Self(iced::widget::button(content))
    }
}
