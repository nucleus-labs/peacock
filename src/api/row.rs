#[derive(Clone)]
pub struct BuilderRow {
    id: Option<&'static str>,
    children: Vec<Box<dyn super::ElementBuilder<'static>>>,
}

impl BuilderRow {
    pub fn new(id: Option<&'static str>, children: Vec<Box<dyn super::ElementBuilder<'static>>>) -> Self {
        Self{
            id,
            children,
        }
    }
}

impl<'a> super::ElementBuilder<'a> for BuilderRow {
    fn build(&self) -> super::Element<'a> {
        let child_elements: Vec<_> = self.children.iter().map(|x| x.build()).collect();
        iced::widget::row(child_elements).into()
    }
    
    fn clone_box(&self) -> Box<dyn super::ElementBuilder<'a>> {
        Box::new(self.clone())
    }
}
