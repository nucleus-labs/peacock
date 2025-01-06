use peacock::api::{ApplicationContext, AsyncHandle};

type AsyncApp = AsyncHandle<ApplicationContext<()>>;

fn main() -> peacock::api::Result {
    let app: AsyncApp = ApplicationContext::new("Basic Peacock App");
    {
        let mut app_guard = app.write().unwrap();
        app_guard.read_css_auto();
        app_guard.read_xml_auto();
    }
    ApplicationContext::run(app)
}
