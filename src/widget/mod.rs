
pub mod container;
pub mod button;
pub mod text;

pub type BoxedWidgetBuilder<State> = Box<dyn ElementBuilder<State>>;

pub trait ElementBuilder<State: Default + 'static>{
    fn build<'a>(&'a self, ctx: &'a super::ApplicationContext<State>) -> super::Element<'a>;
    // fn boxed(&self) -> Box<Self>;
}
