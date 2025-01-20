#![allow(clippy::doc_lazy_continuation)]
#![doc = include_str!("../README.md")]

#[allow(clippy::doc_lazy_continuation)]
#[doc = include_str!("../docs/widget/overview.md")]
pub mod widget;

#[allow(clippy::doc_lazy_continuation)]
#[doc = include_str!("../docs/message/overview.md")]
pub mod message;

pub use minijinja::context;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub type AsyncHandle<T> = Arc<RwLock<T>>;
pub type Element<'a> = iced::Element<'a, message::MessageGeneric>;
pub type Result = std::result::Result<(), Error>;

#[derive(Debug, derive_more::Display, derive_more::From)]
pub enum Error {
    Generic(String),
    Iced(iced::Error),
    PinionTemplate(peacock_pinion::template::Error),
    PinionXml(peacock_pinion::xml::Error),
    PinionGeneric(peacock_pinion::Error),
}

pub struct ApplicationContext<State: Default> {
    title: &'static str,
    state: AsyncHandle<State>,
    event_hooks: HashMap<String, Box<message::MessageReceiver<State>>>,
    
    stylesheet: AsyncHandle<peacock_crest::Stylesheet>,
    templates: AsyncHandle<peacock_pinion::TemplateStore<'static>>,
    xml_trees: AsyncHandle<peacock_pinion::XmlStore>,

    root_id: String,
    widget_registry: HashMap<String, widget::BoxedWidgetBuilder<State>>,
}

fn update<State: Default>(_ctx: &mut ApplicationContext<State>, _msg: message::MessageGeneric) {
    
}

fn view<State: Default + 'static>(ctx: &ApplicationContext<State>) -> Element<'_> {
    ctx.get_widget(&ctx.root_id).unwrap().build(ctx)
}

impl<State: Default + 'static> ApplicationContext<State> {
    pub fn new_with_state(title: &'static str, initial_state: State) -> Self {
        let mut widget_registry: HashMap<String, widget::BoxedWidgetBuilder<State>> = HashMap::new();
        widget_registry.insert("pk-root".into(), widget::container::ContainerBuilder::new(Vec::new()));

        Self{
            title,
            state: Arc::new(RwLock::new(initial_state)),
            event_hooks: HashMap::new(),

            stylesheet: Arc::new(RwLock::new(peacock_crest::Stylesheet::default())),
            templates: peacock_pinion::TemplateStore::new(),
            xml_trees: peacock_pinion::XmlStore::new(),

            root_id: "pk-root".into(),
            widget_registry,
        }
    }

    pub fn new(title: &'static str) -> Self {
        Self::new_with_state(title, State::default())
    }

    pub fn add_css(&mut self, css: &str) -> std::result::Result<(), String> {
        let mut new_style = css.parse::<peacock_crest::Stylesheet>()
            .map_err(|e| format!("Failed to parse CSS: {e}"))?
            .style_rules;

        let mut stylesheet = self.stylesheet.write().map_err(|_| "Failed to acquire write lock")?;
        stylesheet.style_rules.append(&mut new_style);

        Ok(())
    }

    pub fn read_css(&mut self, filepath: &std::path::Path) -> std::result::Result<(), String> {
        let file_contents = std::fs::read_to_string(filepath)
            .map_err(|e| format!("Failed to read CSS file '{filepath:?}': {e}"))?;
        self.add_css(&file_contents)
    }
    
    pub fn read_css_auto(&mut self) -> std::result::Result<(), String> {
        let css_files = glob::glob("static/css/**/*.css").map_err(|e| format!("Glob pattern failed: {e}"))?;

        for css_file_result in css_files {
            match css_file_result {
                Ok(path) => {
                    if let Err(err) = self.read_css(&path) {
                        panic!("Error processing file '{path:?}': {err}");
                    }
                },
                Err(e) => panic!("Glob error: {e}"),
            }
        }

        Ok(())
    }

    pub fn add_xml_template(&mut self, name: &str, xml: &str) -> std::result::Result<(), String> {
        self.templates.write().unwrap().append_raw(name.into(), xml.into())
            .map_err(|e| format!("Failed to process XML source: {e}"))?;

        Ok(())
    }

    pub fn read_xml_add_template(&mut self, name: &str, filepath: &std::path::Path) -> std::result::Result<(), String> {
        self.templates.write().unwrap().append_from_file(name.into(), filepath)
            .map_err(|e| format!("Failed to read XML file '{filepath:?}': {e}"))?;

        Ok(())
    }

    pub fn read_xml_templates_auto(&mut self) -> std::result::Result<(), String> {
        let xml_files = glob::glob("static/xml/**/*.xml").map_err(|e| format!("Glob pattern failed: {e}"))?;

        for xml_file_result in xml_files {
            match xml_file_result {
                Ok(path) => {
                    let name = path.file_stem().unwrap().to_str().unwrap();
                    if let Err(err) = self.read_xml_add_template(name, &path) {
                        panic!("Error processing file '{path:?}': {err}");
                    }
                },
                Err(e) => panic!("Glob error: {e}"),
            }
        }

        self.change_root_element_id("index".to_string());
        Ok(())
    }

    pub(crate) fn register_node_as_widget(&mut self, node: &peacock_pinion::xml::NodeAsync) -> std::result::Result<String, Error> {
        let node_guard = node.read().unwrap();
        let node_id = node_guard.get_attribute("Default", "id")
            .ok_or("Failed to find id attribute".to_string())?;
        let node_name = node_guard.name.to_lowercase();

        match node_name.as_str() {
            "button" => widget::button::ButtonBuilder::from_node(self, node)?,
            "container" => widget::container::ContainerBuilder::from_node(self, node)?,
            "column" => widget::column::ColumnBuilder::from_node(self, node)?,
            "text" | "text-content" => widget::text::TextBuilder::from_node(self, node)?,

            _ => panic!("Unknown element type '{node_name}'")
        };

        Ok(node_id.clone())
    }

    pub fn render_template_to_registry(&mut self, template_name: &str, registry_id: String, context: minijinja::Value) -> Result {
        let template = self.templates.read().unwrap().get(template_name.into());
        let xml = template.read().unwrap().render(context)?;
        let xml_entry = self.xml_trees.write().unwrap().append_from_source(template_name.into(), xml)?;

        let mut toplevel_ids: Vec<String> = Vec::new();
        for node in xml_entry.read().unwrap().nodes.iter() {
            let node_id = self.register_node_as_widget(node)?;
            toplevel_ids.push(node_id);
        }

        self.widget_registry.insert(registry_id, widget::container::ContainerBuilder::new(toplevel_ids));

        Ok(())
    }

    pub fn change_root_element_id(&mut self, id: String) {
        self.root_id = id;
    }

    pub fn get_widget<'a>(&'a self, widget_id: &str) -> Option<&'a widget::BoxedWidgetBuilder<State>> {
        self.widget_registry.get(widget_id)
    }

    pub fn run(self) -> Result {
        let app = iced::application(self.title, update, view);
        match app.run_with(move || (self, iced::Task::none())) {
            Ok(_) => Ok(()),
            Err(err) => Err(err.into()),
        }
    }
}

impl std::error::Error for Error {}

impl<State: Default + 'static> std::default::Default for ApplicationContext<State> {
    fn default() -> Self {
        Self::new("peacock app")
    }
}
