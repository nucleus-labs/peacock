use peacock::ApplicationContext;
use minijinja::context;

#[derive(Default)]
struct MyState {
    // ...
}

fn main() -> peacock::Result {
    let mut app: ApplicationContext<MyState> = ApplicationContext::new("Basic Peacock App");

    app.read_css_auto()?;
    app.read_xml_templates_auto()?;
    app.render_template_to_registry("index", "index".into(), context!{})?;

    app.run()
}
