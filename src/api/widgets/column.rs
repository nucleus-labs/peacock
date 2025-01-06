use crate::api::{Element, ElementBuilder};

#[derive(Clone)]
pub struct BuilderColumn {
    id: Option<&'static str>,
    children: Vec<Box<dyn ElementBuilder<'static>>>,
}

impl BuilderColumn {
    pub fn new(id: Option<&'static str>, children: Vec<Box<dyn ElementBuilder<'static>>>) -> Self {
        Self { id, children }
    }
}

impl<'a> ElementBuilder<'a> for BuilderColumn {
    fn build(&self) -> Element<'a> {
        let child_elements: Vec<_> = self.children.iter().map(|x| x.build()).collect();
        iced::widget::column(child_elements).into()
    }

    fn clone_box(&self) -> Box<dyn ElementBuilder<'a>> {
        Box::new(self.clone())
    }
}
