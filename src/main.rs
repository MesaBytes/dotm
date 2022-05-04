mod app;
mod ui;

use app::App;

fn main() {
    let mut app = App::new();

    app.init();
}
