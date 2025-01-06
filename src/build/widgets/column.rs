use peacock_pinion::xml::NodeAsync;

// use crate::build::{WidgetContext, constants::*};
use crate::build::WidgetContext;

pub fn compose(
    node: NodeAsync,
    widget_collection: &crate::build::WidgetCollection,
) -> WidgetContext {
    let node_guard = node.read().unwrap();

    assert_eq!(node_guard.name, "Column");

    let id: Option<String> = node_guard.attributes.get("id").cloned();
    let content: String = if node_guard.children.is_empty() {
        "".into()
    } else {
        node_guard
            .children
            .iter()
            .map(|child| {
                WidgetContext::from_node(child.clone(), widget_collection).gen_constructor()
            })
            .collect::<Vec<_>>()
            .join(", ")
    };

    let construction: String = match id.clone() {
        Some(_id) => format!(
            "Box::new(widgets::BuilderColumn::new(Some(\"{_id}\"), vec![{}]))",
            content
        ),
        None => format!(
            "Box::new(widgets::BuilderColumn::new(None, vec![{}]))",
            content
        ),
    };

    WidgetContext { construction, id }
}
