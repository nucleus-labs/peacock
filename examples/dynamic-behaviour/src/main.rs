use peacock::ApplicationContext;
use minijinja::context;

#[derive(Default)]
struct MyState {
    count: usize,
}

fn update_count(ctx: &mut ApplicationContext<MyState>, msg: peacock::message::MessageGeneric) {
    ctx.get_state().write().unwrap().count += 1;

    // assumption is safe because widgets send their identifier and if they don't exist that
    // would be incredibly problematic...
    let button = ctx.get_widget(&msg.0).unwrap();
    let button_content_id = button.get_children()[0].clone();
    let button_content = peacock::widget::text::TextBuilder::new(format!("Clicked {} times!", ctx.get_state().read().unwrap().count));
    
    ctx.set_widget(button_content_id, button_content);
}

fn main() -> peacock::Result {
    let mut app: ApplicationContext<MyState> = ApplicationContext::new("Basic Peacock App");

    app.read_xml_templates_auto()?;
    app.render_template_to_registry("index".into(), "index".into(), context!{})?;

    app.register_message_receiver("button".into(), Box::new(update_count));

    app.run()
}
