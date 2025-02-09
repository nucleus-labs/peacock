
pub mod horizontalrule;
pub mod container;
pub mod checkbox;
// not yet implemented!
// pub mod combobox;
pub mod button;
pub mod column;
#[cfg(feature = "canvas")]
pub mod canvas;
pub mod image;
pub mod text;
pub mod row;

/// convenience type alias
pub type BoxedElementBuilder<State> = Box<dyn ElementBuilder<State>>;

/// A local wrapper for an async [`peacock_pinion::xml::XmlNode`]
#[derive(Debug, Clone)]
pub struct DomElementImpl(crate::AsyncHandle<peacock_pinion::xml::XmlNode>);

/// A trait for objects that contain the context necessary for producing an [`iced::Element`],
/// usually by building an object implementing the [`iced::advanced::Widget`] trait first and
/// casting that to an iced Element.
pub trait ElementBuilder<State: 'static> {
    fn build<'a>(&'a self, ctx: &'a super::ApplicationContext<State>) -> super::Element<'a>;
    fn get_child_ids(&self) -> Vec<String>;
    fn get_dom_element(&self) -> DomElementImpl;
}

impl From<peacock_pinion::xml::NodeAsync> for DomElementImpl {
    fn from(value: peacock_pinion::xml::NodeAsync) -> Self {
        Self(value.0)
    }
}

impl peacock_crest::DomElement for DomElementImpl {
    fn get_inline_style(&self) -> peacock_crest::style::CssStyleProperties {
        let inner_handle = self.0.read().unwrap();
        if inner_handle.has_attribute("Default", "style") {
            let css_source = inner_handle.get_attribute("Default", "style").unwrap();
            css_source.parse().expect(&format!("Failed to parse '{css_source}' as css"))
        } else {
            peacock_crest::style::CssStyleProperties::default()
        }
    }

    fn apply_style_properties(&mut self) {
        todo!()
    }
}
