pub mod app;
pub mod core;


fn main() {
    let application = app::App::init().unwrap();
    application.run();
}

