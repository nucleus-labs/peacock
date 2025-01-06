use peacock::api::{widgets, ApplicationContext, AsyncHandle, MessageGeneric, Task};

type AsyncApplication = AsyncHandle<ApplicationContext<u8>>;

fn on_press(app: &mut AsyncApplication, _: MessageGeneric) -> Option<Task> {
    let mut app_guard = app.write().unwrap();

    *app_guard.get_state() += 1;

    let new_content = format!("Pressed {} times!", app_guard.get_state());

    let mut registry = app_guard.widget_registry.write().unwrap();
    registry.insert(
        "text".into(),
        Box::new(widgets::BuilderText::new("text", new_content)),
    );

    None
}

fn main() -> peacock::api::Result {
    let app: AsyncApplication = ApplicationContext::new("Basic Peacock App");
    {
        let mut app_guard = app.write().unwrap();
        app_guard.read_xml_auto();
        app_guard.add_callback("button", on_press);
    }
    ApplicationContext::run(app)
}
