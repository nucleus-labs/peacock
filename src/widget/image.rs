use peacock_crest::DomElement;

pub struct BuilderImage {
    filepath: String,

    dom_element_ref: super::DomElementImpl,

    inline_style: peacock_crest::CssStyleProperties,
}

impl BuilderImage {
    pub fn new(filepath: String, dom_element_ref: super::DomElementImpl) -> Box<Self> {
        let inline_style = dom_element_ref.get_inline_style();
        Self{
            filepath,

            dom_element_ref,

            inline_style,
        }.into()
    }

    pub fn from_node<State: 'static>(ctx: &mut crate::ApplicationContext<State>, node: &peacock_pinion::xml::NodeAsync) -> Result<(), String> {
        let node_guard = node.read().unwrap();
        let node_id = node_guard.get_attribute("Default", "id")
            .ok_or("Failed to find 'id' attribute".to_string())?;
        let node_src = node_guard.get_attribute("Default", "src")
            .ok_or("Failed to find 'src' attribute".to_string())?;

        assert_eq!(node_guard.children.len(), 0, "Images cannot have children!");

        let new = Self::new(node_src, node.clone().into());
        ctx.widget_registry.insert(node_id, new);

        Ok(())
    }
}

impl<State: 'static> super::ElementBuilder<State> for BuilderImage {
    fn build<'a>(&'a self, _: &'a crate::ApplicationContext<State>) -> crate::Element<'a> {
        iced::widget::image(&self.filepath).into()
    }
    
    fn get_child_ids(&self) -> Vec<String> {
        vec![]
    }
    
    fn get_dom_element(&self) -> super::DomElementImpl {
        self.dom_element_ref.clone()
    }
}
