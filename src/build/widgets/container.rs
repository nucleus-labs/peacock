use peacock_pinion::xml::NodeAsync;

use crate::build::WidgetContext;

pub fn compose(
    node: NodeAsync,
    widget_collection: &crate::build::WidgetCollection,
) -> WidgetContext {
    let node_guard = node.read().unwrap();

    assert_eq!(node_guard.name, "Container");
    assert_eq!(
        node_guard.children.len(),
        1,
        "Container must have exactly one child!"
    );
    assert!(
        node_guard.attributes.contains_key("id"),
        "Containers require an `id` attribute!"
    );
    assert!(
        !node_guard.attributes["id"].is_empty(),
        "Containers require a non-empty `id` attribute!"
    );

    let id: String = node_guard.attributes["id"].clone();
    let child = WidgetContext::from_node(node_guard.children[0].clone(), widget_collection);
    let contents = child.gen_constructor();

    let construction = format!("Box::new(widgets::BuilderContainer::new(\"{id}\", {contents}))");

    WidgetContext {
        construction,
        id: Some(id),
    }
}
