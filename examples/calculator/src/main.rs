use peacock::ApplicationContext;
use minijinja::context;

fn main() -> peacock::Result {
    let mut app: ApplicationContext<()> = ApplicationContext::new("Peacock Calculator Example");

    app.read_xml_templates_auto()?;
    app.render_template_to_registry("index".into(), context!{})?;

    app.run()
}
