
pub struct CanvasBuilder {

}

impl CanvasBuilder {

}

impl<State: 'static> super::ElementBuilder<State> for CanvasBuilder {
    fn build<'a>(&'a self, ctx: &'a crate::ApplicationContext<State>) -> crate::Element<'a> {
        todo!()
    }

    fn get_children(&self) -> Vec<String> {
        Vec::new()
    }
}
