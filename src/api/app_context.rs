use glob::glob;
use peacock_pinion::XmlStore;

use std::collections::HashMap;
use std::default::Default;
// use std::ffi::OsString;
use std::fs::read_to_string;
use std::path::Path;
use std::sync::{Arc, RwLock};

use super::{
    update,
    view,
    AsyncHandle,
    // Element,
    ElementBuilder,
    MessageCallback,
    MessageGeneric,
    Task,
};

#[derive(Debug, derive_more::From)]
pub enum Error {
    #[from]
    Iced(iced::Error),
}

pub type Result = std::result::Result<(), Error>;

pub struct ApplicationContext<App: Default + 'static> {
    template_registry: AsyncHandle<peacock_pinion::TemplateStore<'static>>,
    element_tree: AsyncHandle<peacock_pinion::XmlStore>,
    stylesheet: peacock_crest::style::Stylesheet,

    state: App,
    title: &'static str,

    pub callbacks: AsyncHandle<HashMap<String, Vec<Box<MessageCallback<App>>>>>,
    pub widget_registry: AsyncHandle<HashMap<String, Box<dyn ElementBuilder<'static>>>>,

    handle: std::cell::OnceCell<AsyncHandle<Self>>,
}

impl<App: Default> ApplicationContext<App> {
    pub fn new(title: &'static str) -> AsyncHandle<Self> {
        let new = Self {
            #[allow(clippy::arc_with_non_send_sync)]
            template_registry: peacock_pinion::TemplateStore::new(),
            #[allow(clippy::arc_with_non_send_sync)]
            element_tree: XmlStore::new(),
            stylesheet: peacock_crest::style::Stylesheet::default(),

            title,
            state: Default::default(),

            callbacks: Arc::new(RwLock::new(HashMap::new())),

            #[allow(clippy::arc_with_non_send_sync)]
            widget_registry: Arc::new(RwLock::new(HashMap::new())),

            handle: std::cell::OnceCell::new(),
        };

        let handle = Arc::new(RwLock::new(new));
        let _ = handle.write().unwrap().handle.set(handle.clone());

        handle.clone()
    }

    pub fn get_handle(&self) -> AsyncHandle<Self> {
        self.handle.get().unwrap().clone()
    }

    pub fn get_state(&mut self) -> &mut App {
        &mut self.state
    }

    pub fn read_xml_source(&mut self, name: &str, source: String) {
        let template_guard = self.template_registry.read().unwrap();
        let mut env_guard = template_guard.env.write().unwrap();
        env_guard
            .add_template_owned::<String, String>(name.into(), source)
            .unwrap();
    }

    pub fn read_xml_path(&mut self, path: &std::path::Path) {
        assert!(
            path.is_file(),
            "Provided path '{}' does not exist!",
            path.display()
        );
        let xml_file_os = path.as_os_str();
        let source_result = read_to_string(xml_file_os);
        let source =
            source_result.unwrap_or_else(|_| panic!("Failed to read file '{:?}'", xml_file_os));
        let stem = path.file_stem().unwrap().to_str().unwrap();
        self.read_xml_source(stem, source);
    }

    pub fn read_xml_auto(&mut self) {
        if !Path::new("static/xml/").is_dir() {
            panic!("read_xml_auto() reads from source found in `./static/xml`. Please make sure the directory exists and is available.");
        }

        for xml_file_path in glob("static/xml/**/*.xml").expect("Failed to read glob pattern") {
            if xml_file_path.is_err() {
                panic!("Errored while searching for XML files");
            }
            self.read_xml_path(&xml_file_path.unwrap());
        }
    }

    pub fn read_css_source(&mut self, _name: &str, _source: String) {
        // for selector in peacock_crest::style::CssSelectors::parse(&source) {

        // }
    }

    pub fn read_css_path(&mut self, path: &std::path::Path) {
        assert!(
            path.is_file(),
            "Provided path '{}' does not exist!",
            path.display()
        );
        let css_file_os = path.as_os_str();
        let source_result = read_to_string(css_file_os);
        let source =
            source_result.unwrap_or_else(|_| panic!("Failed to read file '{:?}'", css_file_os));
        let stem = path.file_stem().unwrap().to_str().unwrap();
        self.read_css_source(stem, source);
    }

    pub fn read_css_auto(&mut self) {
        if !Path::new("static/css/").is_dir() {
            panic!("read_css_auto() reads from source found in `./static/css`. Please make sure the directory exists and is available.");
        }

        for css_file_path in glob("static/css/**/*.css").expect("Failed to read glob pattern") {
            if let Ok(path) = css_file_path {
                self.read_css_path(&path)
            } else {
                panic!("Errored while searching for CSS files")
            }
        }
    }

    pub fn add_callback(
        &mut self,
        id: &'static str,
        f: impl Fn(&mut AsyncHandle<ApplicationContext<App>>, MessageGeneric) -> Option<Task> + 'static,
    ) {
        let mut callback_guard = self.callbacks.write().unwrap();

        if !callback_guard.contains_key(id) {
            callback_guard.insert(id.to_string(), Vec::new());
        }
        callback_guard.get_mut(id).unwrap().push(Box::new(f));
    }

    pub fn run(ctx: Arc<RwLock<Self>>) -> Result {
        assert!(
            ctx.read()
                .unwrap()
                .template_registry
                .read()
                .unwrap()
                .env
                .read()
                .unwrap()
                .get_template("index")
                .is_ok(),
            "Could not find 'index' element! Do you have an 'index.xml' file?"
        );

        let app = iced::application(ctx.read().unwrap().title, update, view);
        match app.run_with(move || (ctx, iced::Task::none())) {
            Ok(_) => Ok(()),
            Err(err) => Err(err.into()),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Iced(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for Error {}
