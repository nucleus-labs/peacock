
mod gen_widgets;

use std::path::Path;
use std::sync::{Arc, RwLock};
use std::fs::read_to_string;
use std::ffi::OsString;

use gen_widgets::WidgetContext;

const LOCAL_WIDGET_LOOKUP_NAME: &'static str = "widget_lookup";
// const REMOTE_WIDGET_LOOKUP_NAME: &'static str = "remote_widget_lookup";

pub struct WidgetTreeContext {
    xml_store: Arc<RwLock<crate::pinion::xml::Store>>,
    stylesheet: crate::crest::types::Stylesheet,
}

impl WidgetTreeContext {
    pub fn new() -> Self {
        Self{
            xml_store: crate::pinion::xml::Store::new(),
            stylesheet: crate::crest::types::Stylesheet::new(None)
        }
    }

    pub fn parse_xml(&mut self, filepath: OsString) -> Result<(), super::Error> {
        let source = read_to_string(filepath.clone()).unwrap();
        let mut store = self.xml_store.write().unwrap();
        let filepath_string = filepath.into_string().unwrap();
        match store.append_from_source(filepath_string, source) {
            Ok(_) => Ok(()),
            Err(err) => Err(super::Error::PinionError(err)),
        }
    }

    pub fn parse_css(&mut self, filepath: OsString) -> Result<(), super::Error> {
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
                .replace("-", "_")
                ;
            let dependents: Vec<WidgetContext> = xml_guard.nodes.iter()
                .map(|x| WidgetContext::from(x.clone()))
                .collect()
                ;
            
            let mut result: String = format!(r"
use peacock::api::MessageGeneric;

use std::collections::HashMap;
//use std::rc::Rc;
pub fn gen_{name}<'a>(mut {LOCAL_WIDGET_LOOKUP_NAME}: HashMap<String, iced::Element<'a, MessageGeneric>>) -> iced::Element<'a, MessageGeneric> {{
    ()");

            for dep in dependents.iter() {
                result += &format!(";\n{}\n{}", dep.preface, dep.construction);
            }
            result += "\n}";

            results.push(result);
        }

        results.join("\n")
    }
}
