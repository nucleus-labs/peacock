use crate::api::{Element, ElementBuilder};

#[derive(Clone)]
pub struct BuilderText {
    id: &'static str,
    content: String,
}

impl BuilderText {
    pub fn new(id: &'static str, content: String) -> Self {
        Self { id, content }
    }
}

impl<'a> ElementBuilder<'a> for BuilderText {
    fn build(&self) -> Element<'a> {
        iced::widget::text(self.content.clone()).into()
    }

    fn clone_box(&self) -> Box<dyn ElementBuilder<'a>> {
        Box::new(self.clone())
    }
}

// unsafe impl Send for BuilderText {

// }
