
use rseed_app::App;

fn main() {

    let app = App::from_toml_config().unwrap();

    app.run();
}
