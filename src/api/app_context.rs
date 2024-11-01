
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::default::Default;

use super::{
    AsyncHandle,

    MessageGeneric,
    MessageCallback,

    Element,
    ElementBuilder,

    Task,

    TimerManager,
    // Timer,

    view, update,
};

pub struct ApplicationContext<App: Default + 'static> {
    state: App,
    title: &'static str,

    pub callbacks: AsyncHandle<HashMap<&'static str, Vec<Box<MessageCallback<App>>>>>,
    pub widget_registry: AsyncHandle<HashMap<String, Box<dyn ElementBuilder<'static>>>>,
    pub view: Box<dyn Fn(&AsyncHandle<Self>) -> Element<'static>>,

    timers: TimerManager,

    handle: std::cell::OnceCell<AsyncHandle<Self>>,
}

impl<App: Default> ApplicationContext<App> {
    pub fn new(title: &'static str, view: impl Fn(&AsyncHandle<Self>) -> Element<'static> + 'static) -> AsyncHandle<Self> {
        let new = Self {
            title,
            state: Default::default(),

            callbacks: Arc::new(RwLock::new(HashMap::new())),
            view: Box::new(view),

            #[allow(clippy::arc_with_non_send_sync)]
            widget_registry: Arc::new(RwLock::new(HashMap::new())),

            timers: TimerManager::new(),
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

    pub fn add_callback(
        &mut self,
        id: &'static str,
        f: impl Fn(&mut AsyncHandle<ApplicationContext<App>>, MessageGeneric) -> Option<Task> + 'static,
    ) {
        let mut callback_guard = self.callbacks.write().unwrap();

        if !callback_guard.contains_key(id) {
            callback_guard.insert(id, Vec::new());
        }
        callback_guard.get_mut(id).unwrap().push(Box::new(f));
    }

    pub fn run(ctx: Arc<RwLock<Self>>) -> iced::Result {
        let app = iced::application(ctx.read().unwrap().title, update, view);
        app.run_with(move || (ctx, iced::Task::none()))
    }
}
