
mod error;

/// 
/// Compose the source for a WidgetContext from a Pinion Node.
/// 
/// `Button` XML elements require:
///     - an ID - will error if not present
///     - Raw inner text OR one of the following Element types:
///         - raw inner text (cannot be changed dynamically)
///             - `<Button id="counter">Push me!</Button>`
///         - [`text`]
///             - `<Button id="counter"><Text id="counted">Pushed 0 times!</Text></Button>`
///         - [`image`]
///         - [`svg`]
/// 
mod button;
mod canvas;
mod center;
mod checkbox;
mod column;
mod combo_box;
mod container;
mod focus_next;
mod focus_previous;
mod horizontal_rule;
mod horizontal_space;
mod hover;
mod image;
mod keyed_column;
mod markdown;
mod mouse_area;
mod opaque;
mod pane_grid;
mod pick_list;
mod progress_bar;
mod qr_code;
mod radio;
mod responsive;
mod rich_text;
mod row;
mod scrollable;
mod shader;
mod slider;
mod span;
mod stack;
mod svg;
mod text;
mod text_editor;
mod text_input;
mod themer;
mod toggler;
mod tooltip;
mod value;
mod vertical_rule;
mod vertical_slider;
mod vertical_space;

mod lazy;

use peacock_pinion::xml::NodeAsync;
// use super::REMOTE_WIDGET_LOOKUP_NAME;

pub struct WidgetContext {
    pub preface: String,

    pub construction: String,
    pub id: Option<String>,
}

// fn make_delegate(id: String, procedure: String, message_type: String) -> String {
//     format!(r#"
// || -> {message_type} {{
//     // safe assumption because of build-time constraints
//     let remote_widget_procedures = {REMOTE_WIDGET_LOOKUP_NAME}["{id}"];

//     // cannot be assumed because different widgets have different event types
//     if remote_widget_procedures.ContainsKey("{procedure}") {{
//         let procedure = remote_widget_procedures["{procedure}"];
//         return procedure();
//     }}
//     else {{
//         return {message_type}::default();
//     }}
// }}
// "#
//     ).trim().into()
// }

impl From<NodeAsync> for WidgetContext {
    fn from(node: NodeAsync) -> Self {
        let node_guard = node.read().unwrap();
        let name = node_guard.name.clone();

        match name.as_str() {
            "Button" => button::compose(node.clone()),
            "Text" => text::compose(node.clone()),
            // "Canvas" => 
            _ => panic!("Tried to construct a WidgetContext from an unknown Element type '{name}'!")
        }
    }
}
