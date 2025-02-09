use crate::message::{MessageGeneric, MessageGenericInner};
use peacock_crest::DomElement;

pub struct BuilderCheckbox {
    id: String,
    label: String,
    state: bool,

    dom_element_ref: super::DomElementImpl,

    inline_style: peacock_crest::CssStyleProperties,
}

impl BuilderCheckbox {
    pub fn new(id: String, label: String, state: bool, dom_element_ref: super::DomElementImpl) -> Box<Self> {
        let inline_style = dom_element_ref.get_inline_style();
        Self{
            id,
            label,
            state,

            dom_element_ref,

            inline_style,
        }.into()
    }

    pub fn from_node<State: 'static>(ctx: &mut crate::ApplicationContext<State>, node: &peacock_pinion::xml::NodeAsync) -> Result<(), String> {
        let node_guard = node.read().unwrap();
        let node_id = node_guard.get_attribute("Default", "id")
            .ok_or("Failed to find id attribute".to_string())?;
        let node_initial = node_guard.get_attribute("Default", "initial")
            .map(|state| state.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        let node_label = node_guard.get_attribute("Default", "label")
            .ok_or("Failed to find label attribute".to_string())?;
        
        let new = Self::new(node_id.clone(), node_label, node_initial, node.clone().into());
        ctx.widget_registry.insert(node_id, new);

        Ok(())
    }
}

impl<State: 'static> super::ElementBuilder<State> for BuilderCheckbox {
    fn build<'a>(&'a self, _ctx: &'a crate::ApplicationContext<State>) -> crate::Element<'a> {
        iced::widget::checkbox(self.label.clone(), self.state)
            .on_toggle(|to| MessageGeneric(self.id.clone(), MessageGenericInner::Checkbox(to)))
            .into()
    }

    fn get_child_ids(&self) -> Vec<String> {
        Vec::new()
    }
    
    fn get_dom_element(&self) -> super::DomElementImpl {
        self.dom_element_ref.clone()
    }
}
