
pub mod horizontalrule;
pub mod container;
pub mod checkbox;
// not yet implemented!
// pub mod combobox;
pub mod button;
pub mod column;
#[cfg(feature = "canvas")]
pub mod canvas;
pub mod text;
pub mod row;

/// convenience type alias
pub type BoxedElementBuilder<State> = Box<dyn ElementBuilder<State>>;

/// A trait for objects that contain the context necessary for producing an [`iced::Element`],
/// usually by building an object implementing the [`iced::advanced::Widget`] trait first and
/// casting that to an iced Element.
pub trait ElementBuilder<State: 'static> {
    fn build<'a>(&'a self, ctx: &'a super::ApplicationContext<State>) -> super::Element<'a>;
    fn get_children(&self) -> Vec<String>;
}
