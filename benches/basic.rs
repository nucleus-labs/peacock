use criterion::{black_box, criterion_group, criterion_main, Criterion};
use peacock::ApplicationContext;
use minijinja::context;

#[derive(Default)]
struct MyState(usize);

fn benchmark_basic(c: &mut Criterion) {
    let mut app: ApplicationContext<MyState> = ApplicationContext::new("Basic Peacock App");

    app.read_xml_templates_auto().unwrap();
    app.render_template_to_registry("index".into(), "index".into(), context!{}).unwrap();
    app.render_template_to_registry("template".into(), "template".into(), context!{count => 0}).unwrap();

    let mut app = black_box(app);

    c.bench_function("registry_update_builder_contents", |b| {
        b.iter(|| {
            app.get_state().write().unwrap().0 += 1;

            // assumption is safe because widgets send their identifier and if they don't exist that
            // would be incredibly problematic...
            let button = app.get_widget("button1").unwrap();
            let button_content_id = button.get_children()[0].clone();
            let button_content = peacock::widget::text::TextBuilder::new(format!("Clicked {} times!", app.get_state().read().unwrap().0));
            
            app.set_widget(button_content_id, button_content);
        });
    });
    
    c.bench_function("registry_rerender_to", |b| {
        b.iter(|| {
            app.get_state().write().unwrap().0 += 1;
            app.render_template_to_registry("template".into(), "template".into(), context!{count => app.get_state().read().unwrap().0}).unwrap();
        });
    });
}

criterion_group!(benches, benchmark_basic);
criterion_main!(benches);
