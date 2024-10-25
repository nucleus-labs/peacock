
#![allow(dead_code)]

mod button;

use std::collections::HashMap;
use std::default::Default;

pub use button::*;

pub struct Application<State: Default + 'static> {
    state: State,
    title: &'static str,
    view: Box<dyn Fn(HashMap<String, iced::Element<MessageGeneric>>) -> iced::Element<MessageGeneric>>,
    callbacks: HashMap<MessageGeneric, Vec<Box<dyn Fn(&mut State, MessageGeneric) -> iced::Task<MessageGeneric>>>>,
}

fn update<State: Default>(app: &mut Application<State>, m: MessageGeneric) -> iced::Task<MessageGeneric> {
    if !app.callbacks.contains_key(&m) { return ().into(); }

    let mut tasks: Vec<iced::Task<MessageGeneric>> = Vec::new();
    for callback in app.callbacks[&m].iter() {
        tasks.push(callback(&mut app.state, m.clone()));
    }

    iced::Task::batch(tasks)
}

fn view<State: Default>(app: &Application<State>) -> iced::Element<MessageGeneric> {
    (app.view)(HashMap::new())
}

impl<State: Default> Application<State> {
    pub fn new(title: &'static str, view: impl Fn(HashMap<String, iced::Element<MessageGeneric>>) -> iced::Element<MessageGeneric> + 'static) -> Self {
        Self{
            state: Default::default(),
            title,
            view: Box::new(view),
            callbacks: HashMap::new(),
        }
    }

    pub fn add_callback(&mut self, m: MessageGeneric, f: impl Fn(&mut State, MessageGeneric) -> iced::Task<MessageGeneric> + 'static) {
        if !self.callbacks.contains_key(&m) {
            self.callbacks.insert(m.clone(), Vec::new());
        }
        self.callbacks.get_mut(&m).unwrap().push(Box::new(f));
    }

    pub fn run(self) -> iced::Result {
        iced::application(self.title, update, view).run_with(move || (self, iced::Task::none()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MessageGeneric {
    Button(String, MessageButton),
}

pub fn canvas() -> iced::Element<'static, ()> {
    todo!()
}

pub fn center() -> iced::Element<'static, ()> {
    todo!()
}

pub fn checkbox() -> iced::Element<'static, ()> {
    todo!()
}

pub fn column() -> iced::Element<'static, ()> {
    todo!()
}

pub fn combo_box() -> iced::Element<'static, ()> {
    todo!()
}

pub fn container() -> iced::Element<'static, ()> {
    todo!()
}

pub fn focus_next() -> iced::Element<'static, ()> {
    todo!()
}

pub fn focus_previous() -> iced::Element<'static, ()> {
    todo!()
}

pub fn horizontal_rule() -> iced::Element<'static, ()> {
    todo!()
}

pub fn horizontal_space() -> iced::Element<'static, ()> {
    todo!()
}

pub fn hover() -> iced::Element<'static, ()> {
    todo!()
}

pub fn image() -> iced::Element<'static, ()> {
    todo!()
}

pub fn keyed_column() -> iced::Element<'static, ()> {
    todo!()
}

pub fn lazy() -> iced::Element<'static, ()> {
    todo!()
}

pub fn markdown() -> iced::Element<'static, ()> {
    todo!()
}

pub fn mouse_area() -> iced::Element<'static, ()> {
    todo!()
}

pub fn opaque() -> iced::Element<'static, ()> {
    todo!()
}

pub fn pane_grid() -> iced::Element<'static, ()> {
    todo!()
}

pub fn pick_list() -> iced::Element<'static, ()> {
    todo!()
}

pub fn progress_bar() -> iced::Element<'static, ()> {
    todo!()
}

pub fn qr_code() -> iced::Element<'static, ()> {
    todo!()
}

pub fn radio() -> iced::Element<'static, ()> {
    todo!()
}

pub fn responsivelazy() -> iced::Element<'static, ()> {
    todo!()
}

pub fn rich_text() -> iced::Element<'static, ()> {
    todo!()
}

pub fn row() -> iced::Element<'static, ()> {
    todo!()
}

pub fn scrollable() -> iced::Element<'static, ()> {
    todo!()
}

pub fn shaderwgpu() -> iced::Element<'static, ()> {
    todo!()
}

pub fn slider() -> iced::Element<'static, ()> {
    todo!()
}

pub fn span() -> iced::Element<'static, ()> {
    todo!()
}

pub fn stack() -> iced::Element<'static, ()> {
    todo!()
}

pub fn svg() -> iced::Element<'static, ()> {
    todo!()
}

pub fn text_editor() -> iced::Element<'static, ()> {
    todo!()
}

pub fn text_input() -> iced::Element<'static, ()> {
    todo!()
}

pub fn text() -> iced::Element<'static, ()> {
    todo!()
}

pub fn themer() -> iced::Element<'static, ()> {
    todo!()
}

pub fn toggler() -> iced::Element<'static, ()> {
    todo!()
}

pub fn tooltip() -> iced::Element<'static, ()> {
    todo!()
}

pub fn value() -> iced::Element<'static, ()> {
    todo!()
}

pub fn vertical_rule() -> iced::Element<'static, ()> {
    todo!()
}

pub fn vertical_slider() -> iced::Element<'static, ()> {
    todo!()
}

pub fn vertical_space() -> iced::Element<'static, ()> {
    todo!()
}
