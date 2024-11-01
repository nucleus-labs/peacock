use peacock_pinion::xml::NodeAsync;

// use crate::build::{WidgetContext, constants::*};
use crate::build::WidgetContext;

///
/// Compose the source for a WidgetContext from a Pinion Node.
///
/// `Text` XML elements require:
///     - an ID - will error if not present
///     - raw inner text
///
pub fn compose(node: NodeAsync, _widget_collection: &crate::build::WidgetCollection) -> WidgetContext {
    let node_guard = node.read().unwrap();

    assert_eq!(node_guard.name, "Text");
    assert!(
        node_guard.children.len() == 0,
        "Text elements may not contain children!"
    );
    assert!(
        node_guard.attributes.contains_key("id"),
        "Text elements require an `id` attribute!"
    );
    assert!(
        !node_guard.attributes["id"].is_empty(),
        "Text elements require a non-empty `id` attribute!"
    );

    let id = node_guard.attributes["id"].clone();

    let content = node_guard.text_content.clone().unwrap().trim().to_string();
    let construction = format!("Box::new(widgets::BuilderText::new(\"{}\", \"{}\".into()))", id.clone(), content);

    WidgetContext {
        construction,
        id: Some(id),
    }
}
