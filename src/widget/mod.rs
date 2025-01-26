
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

pub type BoxedWidgetBuilder<State> = Box<dyn ElementBuilder<State>>;

pub trait ElementBuilder<State: 'static> {
    fn build<'a>(&'a self, ctx: &'a super::ApplicationContext<State>) -> super::Element<'a>;
    fn get_children(&self) -> Vec<String>;
}
