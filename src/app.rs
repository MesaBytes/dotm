use crate::ui::Ui;

pub struct App;

impl App {
    pub fn new() -> App {
        App
    }

    pub fn init(&self) {
        let mut ui = Ui::new();

        ui.init();
    }
}
