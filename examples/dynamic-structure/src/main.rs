fn main() -> peacock::api::Result {
    let app: AsyncApp = peacock::api::ApplicationContext::new("Dynamic Structure Peacock App");
    {
        let app_guard = app.read().unwrap();
        app_guard.widget_registry.write().unwrap().insert(
            "button".into(),
            Box::new(peacock::api::widgets::BuilderButton::new(
                "button",
                Box::new(peacock::api::widgets::BuilderText::new(
                    "text",
                    "Welcome to the dynamic structure Peacock example!".into(),
                )),
            )),
        );
    }
    peacock::api::ApplicationContext::run(app)
}
