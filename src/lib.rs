#![doc = include_str!("../README.md")]

#[doc = include_str!("../docs/widget/overview.md")]
pub mod widget;

// #[doc = include_str!("../docs/message/overview.md")]
pub mod message;

pub use minijinja::context;
use peacock_pinion::xml::{NodeAsync, XmlNode};

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// a convenience alias
pub type AsyncHandle<T> = Arc<RwLock<T>>;
/// a convenience and simplicity alias
pub type Element<'a> = iced::Element<'a, message::MessageGeneric>;
/// a convenience alias
pub type Result = std::result::Result<(), Error>;

#[derive(Debug, derive_more::Display, derive_more::From)]
pub enum Error {
    Generic(String),
    Iced(iced::Error),
    PinionTemplate(peacock_pinion::template::Error),
    PinionXml(peacock_pinion::xml::Error),
    PinionGeneric(peacock_pinion::Error),
}

#[doc = include_str!("../docs/application_context.md")]
pub struct ApplicationContext<State = ()> {
    title: &'static str,
    state: AsyncHandle<State>,
    event_hooks: HashMap<String, AsyncHandle<Vec<Box<message::MessageReceiver<State>>>>>,
    
    stylesheet: AsyncHandle<peacock_crest::Stylesheet>,
    templates: AsyncHandle<peacock_pinion::TemplateStore<'static>>,
    xml_trees: AsyncHandle<peacock_pinion::XmlStore>,

    root_id: String,
    widget_registry: HashMap<String, widget::BoxedElementBuilder<State>>,
}

fn update<State: 'static>(ctx: &mut ApplicationContext<State>, msg: message::MessageGeneric) {
    if ctx.event_hooks.contains_key(&msg.0) {
        let receivers_handle = &ctx.event_hooks[&msg.0].clone();
        let receivers = receivers_handle.read().unwrap();
        for receiver in receivers.iter() {
            (*receiver)(ctx, msg.clone())
        }
    }
}

fn view<State: 'static>(ctx: &ApplicationContext<State>) -> Element<'_> {
    ctx.get_widget(&ctx.root_id).unwrap().build(ctx)
}

impl<State: 'static> ApplicationContext<State> {
    /// Creates a new context with the given title and default user-defined state object
    pub fn new(title: &'static str) -> Self
    where
        State: Default
    {
        Self::new_with_state(title, State::default())
    }

    /// Creates a new context with the given title and provided initial user-defined state object
    pub fn new_with_state(title: &'static str, initial_state: State) -> Self {
        let mut widget_registry: HashMap<String, widget::BoxedElementBuilder<State>> = HashMap::new();
        let default_node = NodeAsync(Arc::new(RwLock::new(XmlNode::default())));
        widget_registry.insert("pk-root".into(), widget::container::BuilderContainer::new(Vec::new(), default_node.into()));

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

    /// parse the provided css string into a [`peacock_crest::Stylesheet`] and update the
    /// context's application-wide stylesheet
    pub fn add_css(&mut self, css: &str) -> std::result::Result<(), String> {
        let mut new_style = css.parse::<peacock_crest::Stylesheet>()
            .map_err(|e| format!("Failed to parse CSS: {e}"))?
            .style_rules;

        let mut stylesheet = self.stylesheet.write().map_err(|_| "Failed to acquire write lock")?;
        stylesheet.style_rules.append(&mut new_style);

        Ok(())
    }

    /// read the file at the provided path, parse the contents into a [`peacock_crest::Stylesheet`]
    /// and update the context's application-wide stylesheet
    pub fn read_css(&mut self, filepath: &std::path::Path) -> std::result::Result<(), String> {
        let file_contents = std::fs::read_to_string(filepath)
            .map_err(|e| format!("Failed to read CSS file '{filepath:?}': {e}"))?;
        self.add_css(&file_contents)
    }
    
    /// automatically search for *.css files in 'static/css/' and pass the paths to [`Self::read_css`]
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

    /// register the provided xml string with the template registry using `name` as the index
    pub fn add_xml_template(&mut self, name: &str, xml: &str) -> std::result::Result<(), String> {
        self.templates.write().unwrap().append_raw(name.into(), xml.into())
            .map_err(|e| format!("Failed to process XML source: {e}"))?;

        Ok(())
    }

    /// read the file at the provided path and register the contents with the template registry using `name` as the index
    pub fn read_xml_add_template(&mut self, name: &str, filepath: &std::path::Path) -> std::result::Result<(), String> {
        self.templates.write().unwrap().append_from_file(name.into(), filepath)
            .map_err(|e| format!("Failed to read XML file '{filepath:?}': {e}"))?;

        Ok(())
    }

    /// automatically search for *.xml files in 'static/xml/' and pass the paths to [`Self::read_xml_add_template`] with
    /// the file's name as the template index
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

    /// attempt to convert a [`peacock_pinion::xml::NodeAsync`] DOM element to an object implementing the
    /// [`widget::ElementBuilder`] trait based on the node's name
    pub(crate) fn register_node_as_widget(&mut self, node: &peacock_pinion::xml::NodeAsync) -> std::result::Result<String, Error> {
        let node_guard = node.read().unwrap();
        let node_id = node_guard.get_attribute("Default", "id")
            .ok_or("Failed to find id attribute".to_string())?;
        let node_name = node_guard.name.to_lowercase();

        match node_name.as_str() {
            "button" => widget::button::BuilderButton::from_node(self, node)?,
            "container" => widget::container::BuilderContainer::from_node(self, node)?,
            "column" => widget::column::BuilderColumn::from_node(self, node)?,
            "text" | "text-content" => widget::text::BuilderText::from_node(self, node)?,
            "row" => widget::row::BuilderRow::from_node(self, node)?,
            "icon" => widget::image::BuilderImage::from_node(self, node)?,

            _ => panic!("Unknown element type '{node_name}'")
        };

        Ok(node_id.clone())
    }

    /// get the template whose index is `template_name` and render it to xml using `context` as the context.
    /// Then parse the resulting XML string into a DOM element tree, and convert each of the tree's nodes to
    /// an object implementing the [`widget::ElementBuilder`] trait, with the root element being registered
    /// using `registry_id` as the index
    pub fn render_template_to_registry(&mut self, template_name: String, context: minijinja::Value) -> Result {
        let template = self.templates.read().unwrap().get(&template_name);
        let xml = template.read().unwrap().render(context)?;

        let xml_entry = {
            let mut tree_guard = self.xml_trees.write().unwrap();
            if tree_guard.has(&template_name) {
                tree_guard.remove(&template_name);
            }
            tree_guard.append_from_source(template_name, xml)?
        };

        
        let node_handle = xml_entry.read().unwrap();
        let root_handle = node_handle.root.read().unwrap();

        match &root_handle.name.to_lowercase() as &str {
            "body" => {
                let registry_id = root_handle.get_attribute("Default", "route").expect("Root XML objects are required to have a 'route' attribute! See <url> for more information");
                let mut children: Vec<String> = Vec::new();
                for node in root_handle.children.iter() {
                    children.push(self.register_node_as_widget(node)?);
                }
                let default_node = NodeAsync(Arc::new(RwLock::new(XmlNode::default())));
                let root_container = widget::container::BuilderContainer::new(children, default_node.into());
        
                self.widget_registry.insert(registry_id, root_container);
            },
            "modal" => return Err(Error::Generic("Modals are not yet implemented!".into())),

            _ => return Err(Error::Generic("Root object is required to be 'body' or 'modal'!".into())),
        }

        Ok(())
    }

    /// in order to render the context into a GUI application, it references the widget in the widget registry
    /// whose index is that of the context's root id. With this function you can change what ID is used, and as
    /// such can change what gets rendered to the screen
    pub fn change_root_element_id(&mut self, id: String) {
        self.root_id = id;
    }

    /// widgets may define types of events which are emitted as a `message` to the application itself, whether
    /// they're a result of some user interaction or not. To update the state in response to these events, you
    /// must subscribe to that widget's events with a function accepting the correct arguments. The `id` is the
    /// id of the widget that you're subscribing to.
    pub fn register_message_receiver(&mut self, id: String, receiver: Box<message::MessageReceiver<State>>) {
        if !self.event_hooks.contains_key(&id) {
            self.event_hooks.insert(id, Arc::new(RwLock::new(vec![receiver])));
        } else {
            self.event_hooks[&id].write().unwrap().push(receiver);
        }
    }

    pub fn get_widget<'a>(&'a self, widget_id: &str) -> Option<&'a widget::BoxedElementBuilder<State>> {
        self.widget_registry.get(widget_id)
    }

    pub fn set_widget<'a>(&'a mut self, widget_id: String, widget: widget::BoxedElementBuilder<State>) {
        self.widget_registry.insert(widget_id, widget);
    }

    pub fn get_state(&self) -> AsyncHandle<State> {
        self.state.clone()
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
