#![allow(dead_code)]

mod app_context;
mod template;

pub mod widgets;

#[allow(unused_imports)]
pub mod messages {
    pub use super::widgets::MessageButton as Button;
    pub use super::widgets::MessageSlider as Slider;
}

use std::sync::{Arc, OnceLock, RwLock, Weak};

pub use app_context::*;

pub type MessageGeneric = (String, MessageGenericInner);
pub type Element<'a> = iced::Element<'a, MessageGeneric>;
pub type AsyncHandle<T> = Arc<RwLock<T>>;
pub type WeakHandle<T> = Weak<RwLock<T>>;
pub type Task = iced::Task<MessageGeneric>;
pub type MessageCallback<App> =
    dyn Fn(&mut AsyncHandle<ApplicationContext<App>>, MessageGeneric) -> Option<Task>;

#[derive(Debug, Clone)]
pub enum MessageGenericInner {
    Button(messages::Button),
    Slider(messages::Slider),

    // UserDefined(),
}

pub trait ElementBuilder<'a>: Send + Sync {
    // fn apply(&self, stylesheet: peacock_crest::style::CssStyleProperties);
    fn build(&self) -> Element<'a>;
    fn clone_box(&self) -> Box<dyn ElementBuilder<'a>>;

    // fn gen_id() -> String {
    //     format!()
    // }
}

#[inline]
fn view<App: Default>(app: &AsyncHandle<ApplicationContext<App>>) -> Element<'_> {
    static DEFAULT_ROOT: OnceLock<Box<dyn ElementBuilder>> = OnceLock::new();
    app.read()
        .unwrap()
        .widget_registry
        .read()
        .unwrap()
        .get("peacock-root")
        .unwrap_or(DEFAULT_ROOT.get_or_init(|| {
            Box::new(widgets::BuilderText::new(
                "peacock--welcome",
                "Welcome to Peacock!".into(),
            )) as Box<dyn ElementBuilder>
        }))
        .build()
}

fn update<App: Default>(app: &mut AsyncHandle<ApplicationContext<App>>, m: MessageGeneric) -> Task {
    let app_guard = app.write().unwrap();
    let callbacks: Vec<Box<MessageCallback<App>>> = {
        let mut callback_guard = app_guard.callbacks.write().unwrap();

        if !callback_guard.contains_key(&m.0) {
            return ().into();
        }

        callback_guard.remove(&m.0).unwrap()
    };

    let mut tasks: Vec<Task> = Vec::new();
    for callback in callbacks.iter() {
        if let Some(task) = (callback)(&mut app.clone(), m.clone()) {
            tasks.push(task)
        }
    }

    {
        let mut callback_guard = app_guard.callbacks.write().unwrap();
        callback_guard.insert(m.0, callbacks);
    }

    iced::Task::batch(tasks)
}

impl<'a> Clone for Box<dyn ElementBuilder<'a>> {
    fn clone(&self) -> Box<dyn ElementBuilder<'a>> {
        self.clone_box()
    }
}
