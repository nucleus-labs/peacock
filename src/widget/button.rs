
pub struct ButtonBuilder {
    id: String,
    child_id: String,
}

impl ButtonBuilder {
    pub fn new(id: String, child_id: String) -> Box<Self> {
        Self{
            id,
            child_id,
        }.into()
    }

    pub fn from_node<State: Default + 'static>(ctx: &mut crate::ApplicationContext<State>, node: &peacock_pinion::xml::NodeAsync) -> Result<(), String> {
        let node_guard = node.read().unwrap();
        let node_id = node_guard.get_attribute("Default", "id")
            .ok_or("Failed to find id attribute".to_string())?;
        
        if node_guard.children.len() == 1 {
            let mut child_ids: Vec<String> = Vec::new();
            for child in node_guard.children.iter() {
                let child_id = ctx.register_node_as_widget(child)
                    .map_err(|e: crate::Error| format!("Failed to register node: '{e}'"))?;
                child_ids.push(child_id);
            }

            let new = Self::new(node_id.clone(), child_ids[0].clone());
            ctx.widget_registry.insert(node_id, new);

            Ok(())
        } else {
            Err("Buttons must have exactly one text element child or text content".to_string())
        }

    }
}

impl<State: Default + 'static> super::ElementBuilder<State> for ButtonBuilder {
    fn build<'a>(&'a self, ctx: &'a crate::ApplicationContext<State>) -> crate::Element<'a> {
        let child = ctx.get_widget(&self.child_id).unwrap().build(ctx);
        iced::widget::button(child)
            .on_press((self.id.clone(), crate::message::MessageGenericInner::Button))
            .into()
    }
}