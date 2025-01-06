use crate::api::{Element, ElementBuilder, MessageGenericInner};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MessageButton {
    Pressed,
}

#[derive(Clone)]
pub struct BuilderButton {
    id: &'static str,
    content: Box<dyn crate::api::ElementBuilder<'static>>,
}

impl BuilderButton {
    pub fn new(id: &'static str, content: Box<dyn ElementBuilder<'static>>) -> Self {
        Self { id, content }
    }
}

impl<'a> ElementBuilder<'a> for BuilderButton {
    fn build(&self) -> Element<'a> {
        let child = (*self.content).clone_box();
        let child_elem = child.build();
        iced::widget::button(child_elem)
            .on_press((self.id.to_string(), MessageGenericInner::Button(MessageButton::Pressed)))
            .into()
    }

    fn clone_box(&self) -> Box<dyn ElementBuilder<'a>> {
        Box::new(self.clone())
    }
}
