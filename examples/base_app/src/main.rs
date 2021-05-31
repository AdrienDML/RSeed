use rseed_app::App;
fn main() {
    let application = match App::init(
        600, 
        600,
        String::from("Base_app"),
        (0,0,1).into()
    ) {
        Ok(app) => app,
        Err(e) => {
            println!("{:?}", e);
            panic!();
        }
    };
    application.run();
}
