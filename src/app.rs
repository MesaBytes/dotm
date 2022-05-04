use ncurses as nc;

pub struct App {
    quit: bool,
}

impl App {
    pub fn new() -> App {
        App { quit: false }
    }

    pub fn init(&mut self) {
        nc::initscr();

        while !self.quit {
            for i in 0..10 {
                nc::mv(i, 0);
                nc::addstr(&i.to_string());
            }
            nc::refresh();
            match nc::getch() as u8 as char {
                'q' => {
                    self.quit = true;
                }
                _ => {}
            }
        }

        nc::endwin();
    }
}
