mod constants;
mod widgets;

use peacock_crest::types::Stylesheet;
use peacock_pinion::xml::NodeAsync;
use glob::glob;

use std::fs::{File, remove_file, read_to_string};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::ffi::OsString;
use std::path::Path;
use std::io::Write;

use constants::*;
use widgets::*;

pub type WidgetCollection = HashMap<&'static str, Box<dyn Fn(NodeAsync) -> Option<WidgetContext>>>;

#[derive(Debug)]
pub enum Error {
    PinionError(peacock_pinion::xml::Error),
    CrestError(peacock_crest::Error),
}

pub fn build_with_libraries(widget_libraries: WidgetCollection) {
    let mut ctx = XmlTreeContext::new(widget_libraries);

    if !Path::new("static/xml/").is_dir() || !Path::new("static/css/").is_dir() {
        panic!("Peacock builds from source found in `./static/xml` and `./static/css`. Please make sure both directories exist and are available.");
    }
    for xml_file in glob("static/xml/**/*.xml").expect("Failed to read glob pattern") {
        if xml_file.is_err() {
            panic!("Errored while searching for XML files");
        }
        let os_xml_file: OsString = xml_file.unwrap().into();
        
        let result = ctx.parse_xml(os_xml_file.clone());
        if result.is_err() {
            panic!("Failed to parse XML file: {:?}", result.unwrap_err());
        }

        println!("cargo::rerun-if-changed={}", os_xml_file.into_string().unwrap());
    }
    for css_file in glob("static/css/**/*.css").expect("Failed to read glob pattern") {
        if css_file.is_err() {
            panic!("Errored while searching for CSS files");
        }
        let os_css_file: OsString = css_file.unwrap().into();
        
        let result = ctx.parse_css(os_css_file.clone());
        if result.is_err() {
            panic!("Failed to parse CSS file: {:?}", result.unwrap_err());
        }

        println!("cargo::rerun-if-changed={}", os_css_file.into_string().unwrap());
    }

    let gen_os_str: OsString = format!("{}/{}", std::env::var("OUT_DIR").unwrap(), "widgets-gen.rs").into();
    let gen_path = Path::new(&gen_os_str);
    if gen_path.exists() {
        remove_file(gen_path).unwrap();
    }

    File::create_new(gen_path).unwrap().write_all(ctx.gen_source().as_bytes()).unwrap();
}

pub fn build() {
    build_with_libraries(HashMap::new());
}

#[derive(Debug, Clone)]
pub struct WidgetContext {
    construction: String,
    id: Option<String>,
}

pub struct XmlTreeContext {
    xml_store: Arc<RwLock<peacock_pinion::xml::Store>>,
    stylesheet: Stylesheet,
    widget_collection: WidgetCollection,
}

impl WidgetContext {
    fn gen_constructor(&self) -> String {
        match self.id.clone() {
            Some(id) => format!("{LOCAL_WIDGET_LOOKUP_NAME}.remove(\"{id}\").unwrap_or({})", self.construction),
            None => self.construction.clone(),
        }
    }

    pub fn from_node(node: NodeAsync, widget_collection: &WidgetCollection) -> Self {
        let node_guard = node.read().unwrap();
        let name = node_guard.name.clone();

        match name.as_str() {
            "Button"    => button::compose(node.clone(), widget_collection),
            "Text"      => text::compose(node.clone(), widget_collection),
            "Column"    => column::compose(node.clone(), widget_collection),
            "Container" => container::compose(node.clone(), widget_collection),
            "Row"       => row::compose(node.clone(), widget_collection),
            "Slider"    => slider::compose(node.clone(), widget_collection),
            _ => {
                // TODO: Allow registration of new functions that check widget types, to extend the
                // widget library. Then here, iterate over all of them. Also, to manage conflicts,
                // use of loaded extensions must be denoted by a namespace in the XML source.
                // EG. <MyExtension:MyType id="peacock-go-brrrrrrrr" ... />

                if node_guard.namespace.is_none() {
                    panic!("Tried to construct a WidgetContext from an unknown Element type '{name}'!")
                }
                else {
                    let namespace = node_guard.namespace.as_ref().unwrap().as_str();
                    if !widget_collection.contains_key(namespace) {
                        panic!("Tried to construct a WidgetContext from an Element type '{name}' using unknown namespace '{namespace}'!")
                    }
                    else {
                        let collection_result = widget_collection[namespace](node.clone());
                        match collection_result {
                            None => panic!("Widget library '{namespace}' failed to construct a WidgetContext of type '{name}'!"),
                            Some(widget_ctx) => widget_ctx,
                        }
                    }
                }
            }
        }
    }
}

impl XmlTreeContext {
    pub fn new(widget_collection: WidgetCollection) -> Self {
        Self {
            xml_store: peacock_pinion::xml::Store::new(),
            stylesheet: peacock_crest::types::Stylesheet::new(None),
            widget_collection,
        }
    }

    pub fn parse_xml(&mut self, filepath: OsString) -> Result<(), Error> {
        let source = read_to_string(filepath.clone()).unwrap();
        let mut store = self.xml_store.write().unwrap();
        let filepath_string = filepath.into_string().unwrap();
        match store.append_from_source(filepath_string, source) {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::PinionError(err)),
        }
    }

    pub fn parse_css(&mut self, filepath: OsString) -> Result<(), Error> {
        let source = read_to_string(filepath.clone()).unwrap();
        self.stylesheet.parse(source);
        Ok(())
    }

    pub fn gen_source(self) -> String {
        let mut results: Vec<String> = Vec::new();
        let store_guard = self.xml_store.read().unwrap();
        let indices_guard = store_guard.indices.read().unwrap();
        for xml_handle in indices_guard.values() {
            let xml_guard = xml_handle.read().unwrap();
            let name = Path::new(&xml_guard.index)
                .file_stem()
                .unwrap()
                .to_os_string()
                .into_string()
                .unwrap()
                .replace("-", "_");
            let dependents: Vec<WidgetContext> = xml_guard
                .nodes
                .iter()
                .map(|x| WidgetContext::from_node(x.clone(), &self.widget_collection))
                .collect();

            let mut result: String = format!(
                r"
#[allow(unused_imports)]

// THIS FILE IS GENERATED BY THE PEACOCK CRATE
// DO NOT EDIT

use peacock::api::*;

pub fn gen_{name}<State: Default>(app: &AsyncHandle<ApplicationContext<State>>) -> Element<'static> {{
    let app_guard = app.read().unwrap();
    let mut {LOCAL_WIDGET_LOOKUP_NAME} = app_guard.widget_registry.read().unwrap().clone();"
            );

            let last = dependents.last().unwrap();
            for dep in dependents.iter().filter(|&x| !std::ptr::eq(x, last)) {
                result += &format!("\n\t{};", dep.gen_constructor());
            }
            result += &format!("\n\t{}.build().into()", last.gen_constructor());
            result += "\n}";

            results.push(result);
        }

        results.join("\n")
    }
}

impl From<peacock_crest::Error> for Error {
    fn from(value: peacock_crest::Error) -> Self {
        Self::CrestError(value)
    }
}
