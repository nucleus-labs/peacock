
use peacock::api::{ApplicationContext, MessageGeneric, Task, widgets as pwidgets, AsyncHandle};

mod widgets {
    include!(concat!(env!("OUT_DIR"), "/widgets-gen.rs"));
}

#[derive(Default)]
struct AppState {
    press_count: usize,
}

fn on_press(app: &mut AsyncHandle<ApplicationContext<AppState>>, _message: MessageGeneric) -> Option<Task> {
    let mut app_guard = app.write().unwrap();

    app_guard.get_state().press_count += 1;

    let press_count = app_guard.get_state().press_count;
    let mut registry = app_guard.widget_registry.write().unwrap();
    let new_content = format!("Pressed {press_count} times!");
    registry.insert("text".into(), Box::new(pwidgets::BuilderText::new("text", new_content)));
    
    None
}

fn main() -> iced::Result {
    let app = ApplicationContext::<AppState>::new("Basic Peacock App", widgets::gen_index);
    {
        let mut app_guard = app.write().unwrap();
        app_guard.add_callback("button", on_press);
    }
    ApplicationContext::run(app)
}
