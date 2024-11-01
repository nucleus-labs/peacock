
mod widgets {
    include!(concat!(env!("OUT_DIR"), "/widgets-gen.rs"));
}

fn main() -> iced::Result {
    let mut scene = scene::Scene::new();
    scene.add_model(scene::Model::load_obj(std::path::Path::new("static/obj/teapot.obj")));

    let app = peacock::api::ApplicationContext::<()>::new("Peacock Rendering Example", widgets::gen_index);
    {
        let app_handle = app.read().unwrap();
        app_handle.widget_registry.write().unwrap().insert("teapot-renderer".into(), Box::new(scene));
    }
    peacock::api::ApplicationContext::run(app)
}
