use peacock_crest::DomElement;

pub struct BuilderText {
    contents: String,

    dom_element_ref: super::DomElementImpl,

    inline_style: peacock_crest::CssStyleProperties,
}

impl BuilderText {
    pub fn new(contents: String, dom_element_ref: super::DomElementImpl) -> Box<Self> {
        let inline_style = dom_element_ref.get_inline_style();
        Self{
            contents,

            dom_element_ref,

            inline_style,
        }.into()
    }

    pub fn from_node<State: 'static>(ctx: &mut crate::ApplicationContext<State>, node: &peacock_pinion::xml::NodeAsync) -> Result<(), String> {
        let node_guard = node.read().unwrap();

        let node_id = node_guard.get_attribute("Default", "id")
            .ok_or("Failed to find id attribute".to_string())?;
        let new = if node_guard.has_attribute("Default", "content") {
            Self::new(node_guard.get_attribute("Default", "content").unwrap(), node.clone().into())
        } else {
            if node_guard.children.is_empty() {
                Self::new(Default::default(), node.clone().into())
            } else {
                let content = node_guard.children.iter()
                    .filter(|child| {
                        ["text".to_string(), "text-content".to_string()].contains(&child.read().unwrap().name)
                    })
                    .map(|child| child.read().unwrap().get_attribute("Default", "content").expect("Failed to find 'content' attribute"))
                    .collect::<Vec<String>>()
                    .join(" ");

                Self::new(content, node.clone().into())
            }
        };

        ctx.widget_registry.insert(node_id, new);

        Ok(())
    }
}

impl<State: 'static> super::ElementBuilder<State> for BuilderText {
    fn build<'a>(&'a self, _ctx: &'a crate::ApplicationContext<State>) -> crate::Element<'a> {
        iced::widget::text(&self.contents).into()
    }
    
    fn get_child_ids(&self) -> Vec<String> {
        Vec::new()
    }
    
    fn get_dom_element(&self) -> super::DomElementImpl {
        self.dom_element_ref.clone()
    }
}
