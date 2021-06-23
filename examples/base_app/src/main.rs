use rseed_app::{App, Backend};
fn main() {
    let application = App::init(
        600, 
        600,
        String::from("Base_app"),
        (0,0,1).into(),
        Backend::VK,
    ).unwrap();
    application.run();
}
