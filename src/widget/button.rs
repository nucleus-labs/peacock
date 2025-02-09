use peacock_crest::style::prop_validation::overflow::KeywordOverflow;
use peacock_crest::DomElement;

use crate::message::{MessageGeneric, MessageGenericInner};

pub struct BuilderButton {
    id: String,
    child_id: String,

    dom_element_ref: super::DomElementImpl,

    inline_style: peacock_crest::CssStyleProperties,
}

impl BuilderButton {
    pub fn new(id: String, child_id: String, dom_element_ref: super::DomElementImpl) -> Box<Self> {
        let inline_style = dom_element_ref.get_inline_style();
        Self{
            id,
            child_id,

            dom_element_ref,

            inline_style,
        }.into()
    }

    pub fn from_node<State: 'static>(ctx: &mut crate::ApplicationContext<State>, node: &peacock_pinion::xml::NodeAsync) -> Result<(), String> {
        let node_guard = node.read().unwrap();
        let node_id = node_guard.get_attribute("Default", "id")
            .ok_or("Failed to find 'id' attribute".to_string())?;
        
        if node_guard.children.len() == 1 {
            let mut child_ids: Vec<String> = Vec::new();
            for child in node_guard.children.iter() {
                let child_id = ctx.register_node_as_widget(child)
                    .map_err(|e: crate::Error| format!("Failed to register node: '{e}'"))?;
                child_ids.push(child_id);
            }

            let new = Self::new(node_id.clone(), child_ids[0].clone(), node.clone().into());
            ctx.widget_registry.insert(node_id, new);

            Ok(())
        } else {
            Err("Buttons must have exactly one text element child or text content".to_string())
        }
    }
}

impl<State: 'static> super::ElementBuilder<State> for BuilderButton {
    fn build<'a>(&'a self, ctx: &'a crate::ApplicationContext<State>) -> crate::Element<'a> {
        let child = ctx.get_widget(&self.child_id).unwrap().build(ctx);
        let mut widget = iced::widget::button(child)
            .on_press(MessageGeneric(self.id.clone(), MessageGenericInner::Button))
            .clip(true);

        let mut use_container = false;
        let mut use_scroller = false;

        if let Some(overflow) = self.inline_style.eval_prop("overflow-x") {
            match overflow {
                peacock_crest::style::properties::CssStyleProperty::OverflowX(css_attribute_value) => {
                    match css_attribute_value {
                        peacock_crest::CssAttributeValue::Keyword(keyword) => {
                            match keyword {
                                KeywordOverflow::Hidden | KeywordOverflow::Clip => widget = widget.clip(true),
                                KeywordOverflow::Visible => todo!(),
                                KeywordOverflow::Scroll => use_scroller = true,
                                KeywordOverflow::Auto => todo!(),
                            }
                        },
                        _ => unreachable!()
                    }
                },
                _ => unreachable!()
            }
        }

        widget.into()
    }
    
    fn get_child_ids(&self) -> Vec<String> {
        vec![self.child_id.clone()]
    }
    
    fn get_dom_element(&self) -> super::DomElementImpl {
        self.dom_element_ref.clone()
    }
}
