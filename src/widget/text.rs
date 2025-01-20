
pub struct TextBuilder {
    contents: String,
}

impl TextBuilder {
    pub fn new(contents: String) -> Box<Self> {
        Self{
            contents,
        }.into()
    }

    pub fn from_node<State: Default + 'static>(ctx: &mut crate::ApplicationContext<State>, node: &peacock_pinion::xml::NodeAsync) -> Result<(), String> {
        let node_guard = node.read().unwrap();

        let node_id = node_guard.get_attribute("Default", "id")
            .ok_or("Failed to find id attribute".to_string())?;
        let node_text_content = node_guard.get_attribute("Default", "content")
            .ok_or("Failed to find content attribute".to_string())?;

        let new = Self::new(node_text_content);
        ctx.widget_registry.insert(node_id, new);

        Ok(())
    }
}

impl<State: Default + 'static> super::ElementBuilder<State> for TextBuilder {
    fn build<'a>(&'a self, _ctx: &'a crate::ApplicationContext<State>) -> crate::Element<'a> {
        iced::widget::text(&self.contents).into()
    }
}
