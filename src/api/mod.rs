#![allow(dead_code)]

mod app_context;
mod timer;

mod button;
mod column;
mod container;
mod row;
mod slider;
mod text;

use timer::TimerManager;

#[allow(unused_imports)]
pub mod widgets {
    pub use super::button::BuilderButton;
    pub use super::column::BuilderColumn;
    pub use super::container::BuilderContainer;
    pub use super::row::BuilderRow;
    pub use super::slider::BuilderSlider;
    pub use super::text::BuilderText;
}

#[allow(unused_imports)]
pub mod messages {
    pub use super::button::MessageButton as Button;
    pub use super::slider::MessageSlider as Slider;
}

use std::sync::{Arc, RwLock, Weak};

pub use app_context::*;

pub type MessageGeneric = (&'static str, MessageGenericInner);
pub type Element<'a> = iced::Element<'a, MessageGeneric>;
pub type AsyncHandle<T> = Arc<RwLock<T>>;
pub type WeakHandle<T> = Weak<RwLock<T>>;
pub type Task = iced::Task<MessageGeneric>;
pub type MessageCallback<App> = dyn Fn(&mut AsyncHandle<ApplicationContext<App>>, MessageGeneric) -> Option<Task>;

#[derive(Debug, Clone)]
pub enum MessageGenericInner {
    Button(messages::Button),
    Slider(messages::Slider),
}

pub trait ElementBuilder<'a> {
    fn build(&self) -> Element<'a>;
    fn clone_box(&self) -> Box<dyn ElementBuilder<'a>>;
}

fn view<App: Default>(app: &AsyncHandle<ApplicationContext<App>>) -> Element {
    (app.read().unwrap().view)(app)
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
        match callback(&mut app.clone(), m.clone()) {
            Some(task) => tasks.push(task),
            None => (),
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
