#[derive(Clone)]
pub struct BuilderContainer {
    id: &'static str,
    content: Box<dyn super::ElementBuilder<'static>>,
}

impl BuilderContainer {
    pub fn new(id: &'static str, content: Box<dyn super::ElementBuilder<'static>>) -> Self {
        Self{
            id,
            content,
        }
    }
}

impl<'a> super::ElementBuilder<'a> for BuilderContainer {
    fn build(&self) -> super::Element<'a> {
        let child = self.content.clone_box();
        iced::widget::container(child.build()).into()
    }
    
    fn clone_box(&self) -> Box<dyn super::ElementBuilder<'a>> {
        Box::new(self.clone())
    }
}

