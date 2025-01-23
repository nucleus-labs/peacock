
#[derive(Debug)]
pub struct RowBuilder {
    pub children: Vec<String>,
}

impl RowBuilder {
    pub fn new(children: Vec<String>) -> Box<Self> {
        Self{
            children,
        }.into()
    }

    pub fn from_node<State: Default + 'static>(ctx: &mut crate::ApplicationContext<State>, node: &peacock_pinion::xml::NodeAsync) -> Result<(), String> {
        let node_guard = node.read().unwrap();
        let node_id = node_guard.get_attribute("Default", "id")
            .ok_or("Failed to find id attribute".to_string())?;
        
        let mut child_ids: Vec<String> = Vec::new();
        for child in node_guard.children.iter() {
            let child_id = ctx.register_node_as_widget(child)
                .map_err(|e: crate::Error| format!("Failed to register node: '{e}'"))?;
            child_ids.push(child_id);
        }

        let new = Self::new(child_ids);
        ctx.widget_registry.insert(node_id, new);

        Ok(())
    }
}

impl<State: Default + 'static> super::ElementBuilder<State> for RowBuilder {
    fn build<'a>(&'a self, ctx: &'a crate::ApplicationContext<State>) -> crate::Element<'a> {
        let children = self.children.iter()
            .map(|child_id| ctx.get_widget(child_id as &str).unwrap())
            .map(|builder| builder.build(ctx))
            .collect::<Vec<crate::Element<'a>>>();
        iced::widget::column(children).into()
    }
    
    fn get_children(&self) -> Vec<String> {
        self.children.clone()
    }
}
