mod app;
mod ui;

use app::App;

fn main() {
    let app = App::new();

    app.init();
}
