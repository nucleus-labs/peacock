
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MessageButton {
    Pressed,
}

#[derive(Clone)]
pub struct BuilderButton {
    id: &'static str,
    content: Box<dyn super::ElementBuilder<'static>>,
}

impl BuilderButton {
    pub fn new(id: &'static str, content: Box<dyn super::ElementBuilder<'static>>) -> Self {
        Self{
            id,
            content,
        }
    }
}

impl<'a> super::ElementBuilder<'a> for BuilderButton {
    fn build(&self) -> super::Element<'a> {
        let child = (*self.content).clone_box();
        let child_elem = child.build();
        iced::widget::button(child_elem).on_press((self.id, super::MessageGenericInner::Button(MessageButton::Pressed))).into()
    }
    
    fn clone_box(&self) -> Box<dyn super::ElementBuilder<'a>> {
        Box::new(self.clone())
    }
}
