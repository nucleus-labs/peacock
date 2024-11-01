use peacock_pinion::xml::NodeAsync;

use crate::build::WidgetContext;

pub fn compose(node: NodeAsync, _widget_collection: &crate::build::WidgetCollection) -> WidgetContext {
    let node_guard = node.read().unwrap();

    assert_eq!(node_guard.name, "Slider");
    assert!(
        node_guard.children.is_empty(),
        "Sliders cannot have children!"
    );
    assert!(
        node_guard.attributes.contains_key("id"),
        "Sliders require an `id` attribute!"
    );
    assert!(
        !node_guard.attributes["id"].is_empty(),
        "Sliders require a non-empty `id` attribute!"
    );
    assert!(
        node_guard.attributes.contains_key("min"),
        "Sliders require an `min` attribute!"
    );
    assert!(
        node_guard.attributes.contains_key("max"),
        "Sliders require an `max` attribute!"
    );

    let id = node_guard.attributes["id"].clone();
    let min = node_guard.attributes["min"].clone();
    let max = node_guard.attributes["max"].clone();

    let min_num = min.parse::<f32>().expect(format!("Failed to parse '{min}' as a number").as_str());
    let max_num = max.parse::<f32>().expect(format!("Failed to parse '{max}' as a number").as_str());
    let default_num = if node_guard.attributes.contains_key("default") {
        let default = node_guard.attributes["default"].clone();
        default.parse::<f32>().expect(format!("Failed to parse '{default}' as a number").as_str())
    }
    else {
        min_num.clone()
    };

    let construction = format!("Box::new(widgets::BuilderSlider::new(\"{id}\", {min_num}f32, {max_num}f32, {default_num}f32))");

    WidgetContext {
        construction,
        id: Some(id),
    }
}
