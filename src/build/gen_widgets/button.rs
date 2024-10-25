
use peacock_pinion::xml::NodeAsync;

use super::{WidgetContext, super::LOCAL_WIDGET_LOOKUP_NAME};

const VALID_CONTENTS: &[&'static str] = &["Text", "Image", "SVG"];

pub fn compose(node: NodeAsync) -> WidgetContext {
    let node_guard = node.read().unwrap();

    assert_eq!(node_guard.name, "Button");
    assert!(node_guard.children.len() < 2, "Buttons may only contain 0 or 1 children!");
    assert!(node_guard.attributes.contains_key("id"), "Buttons require an `id` attribute!");
    assert!(!node_guard.attributes["id"].is_empty(), "Buttons require a non-empty `id` attribute!");

    let id = node_guard.attributes["id"].clone();
    let mut preface: String;
    let button_content: String;
    if node_guard.children.is_empty() {
        assert!(node_guard.text_content.is_some());
        assert!(!node_guard.text_content.as_ref().unwrap().is_empty());

        preface = String::new();
        button_content = format!(r#""{}""#, node_guard.text_content.clone().unwrap());
    }
    else {
        let child_guard = node_guard.children[0].read().unwrap();
        assert!(
            VALID_CONTENTS.contains(&child_guard.name.as_str()),
            "Invalid child of Button: '{}'", child_guard.name
        );
        let child = WidgetContext::from(node_guard.children[0].clone());
        button_content = child.construction.clone();

        preface = match child.id {
            Some(_) => child.preface.clone(),
            None => String::new(),
        };
    }

    preface += &format!(
        "\n    let widget = iced::widget::button({button_content}).on_press(MessageGeneric::Button(\"{id}\".into(), peacock::api::MessageButton::Pressed));{}",
        format!("\n    {LOCAL_WIDGET_LOOKUP_NAME}.insert(\"{id}\".into(), widget.into());")
    );
    let construction = format!("{LOCAL_WIDGET_LOOKUP_NAME}.remove(\"{id}\").unwrap()");

    WidgetContext{
        construction,
        preface,
        id: Some(id),
    }
}
