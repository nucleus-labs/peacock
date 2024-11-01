
mod widgets {
    include!(concat!(env!("OUT_DIR"), "/widgets-gen.rs"));
}

fn main() -> iced::Result {
    let app = peacock::api::ApplicationContext::<()>::new("Basic Peacock App", widgets::gen_index::<()>);
    peacock::api::ApplicationContext::run(app)
}
