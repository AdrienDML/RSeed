use rseed_app::App;
fn main() {
    let application = App::init(600, 600).unwrap();
    application.run();
}
