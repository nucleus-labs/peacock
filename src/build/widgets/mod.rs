pub mod error;

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
pub mod button;
pub mod center;
pub mod checkbox;
pub mod column;
pub mod combo_box;
pub mod container;
pub mod focus_next;
pub mod focus_previous;
pub mod horizontal_rule;
pub mod horizontal_space;
pub mod hover;
pub mod image;
pub mod keyed_column;
pub mod markdown;
pub mod mouse_area;
pub mod opaque;
pub mod pane_grid;
pub mod pick_list;
pub mod progress_bar;
pub mod qr_code;
pub mod radio;
pub mod responsive;
pub mod rich_text;
pub mod row;
pub mod scrollable;
pub mod slider;
pub mod span;
pub mod stack;
pub mod svg;
pub mod text;
pub mod text_editor;
pub mod text_input;
pub mod themer;
pub mod toggler;
pub mod tooltip;
pub mod value;
pub mod vertical_rule;
pub mod vertical_slider;
pub mod vertical_space;

pub mod lazy;

// pub(crate) fn gen_widgets(node: NodeAsync) ->  {

// }
