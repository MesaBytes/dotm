use crate::ui::Ui;

pub struct App;

impl App {
    pub fn new() -> App {
        App
    }

    pub fn init(&self) {
        // Before initalization ui (list)
        // Display main menu and if 'List' is selected initialize 'List'
        // if 'Backup' is selected run backup function with progress bar
        let ui = Ui::new();

        ui.init();
    }
}
