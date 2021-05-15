use rseed_app::{App, AppError};
fn main() {
    let application = match App::init(600, 600) {
        Ok(app) => app,
        Err(e) => {
            println!("{:?}", e);
            panic!();
        }
    };
    application.run();
}
