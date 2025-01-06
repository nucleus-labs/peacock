use crate::api::{Element, ElementBuilder};

#[derive(Clone)]
pub struct BuilderContainer {
    id: &'static str,
    content: Box<dyn ElementBuilder<'static>>,
}

impl BuilderContainer {
    pub fn new(id: &'static str, content: Box<dyn ElementBuilder<'static>>) -> Self {
        Self { id, content }
    }
}

impl<'a> ElementBuilder<'a> for BuilderContainer {
    fn build(&self) -> Element<'a> {
        let child = self.content.clone_box();
        iced::widget::container(child.build()).into()
    }

    fn clone_box(&self) -> Box<dyn ElementBuilder<'a>> {
        Box::new(self.clone())
    }
}
