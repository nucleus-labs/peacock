use peacock_pinion::xml::NodeAsync;

use crate::build::WidgetContext;

const VALID_CONTENTS: &[&'static str] = &["Text", "Image", "SVG"];

pub fn compose(node: NodeAsync, widget_collection: &crate::build::WidgetCollection) -> WidgetContext {
    let node_guard = node.read().unwrap();

    assert_eq!(node_guard.name, "Button");
    assert!(
        node_guard.children.len() < 2,
        "Buttons cannot have more than one child!"
    );
    assert!(
        node_guard.attributes.contains_key("id"),
        "Buttons require an `id` attribute!"
    );
    assert!(
        !node_guard.attributes["id"].is_empty(),
        "Buttons require a non-empty `id` attribute!"
    );

    let id = node_guard.attributes["id"].clone();

    let button_content: String = if node_guard.children.is_empty() {
        assert!(node_guard.text_content.is_some());
        assert!(!node_guard.text_content.as_ref().unwrap().is_empty());

        format!(r#""{}""#, node_guard.text_content.clone().unwrap().trim())
    } else {
        let child_guard = node_guard.children[0].read().unwrap();
        assert!(
            VALID_CONTENTS.contains(&child_guard.name.as_str()),
            "Invalid child of Button: '{}'",
            child_guard.name
        );
        let child = WidgetContext::from_node(node_guard.children[0].clone(), widget_collection);
        match child.clone().id {
            Some(_) => child.gen_constructor(),
            None => child.construction,
        }
    };

    let construction = format!("Box::new(widgets::BuilderButton::new(\"{id}\", {button_content}))");

    WidgetContext {
        construction,
        id: Some(id),
    }
}
