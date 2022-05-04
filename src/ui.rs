use nc::{getmaxyx, COLOR_PAIR};
use ncurses as nc;

const REGULAR_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;

pub struct Ui {
    quit: bool,
}

impl Ui {
    pub fn new() -> Ui {
        Ui { quit: false }
    }

    pub fn init(&mut self) {
        let mut max_rows: i32 = 0;
        let mut max_columns: i32 = 0;
        let mut cursor_position: usize = 0;

        let window = nc::initscr();

        nc::start_color();
        nc::init_pair(REGULAR_PAIR, nc::COLOR_CYAN, nc::COLOR_BLACK);
        nc::init_pair(HIGHLIGHT_PAIR, nc::COLOR_BLACK, nc::COLOR_WHITE);

        nc::noecho();
        nc::cbreak();
        nc::raw();

        getmaxyx(window, &mut max_rows, &mut max_columns);

        let mut test_items: Vec<i32> = Vec::new();

        for i in 0..5 {
            test_items.push(i);
        }

        let mut last_index: i32 = 0;
        let mut page_number: i32 = 0;

        while !self.quit {
            for (current_index, item) in test_items.iter().enumerate() {
                // if current_index as i32 == max_rows {
                //     last_index = current_index as i32;
                //     page_number += 1;
                //     break;
                // }

                let pair = {
                    if cursor_position == current_index {
                        HIGHLIGHT_PAIR
                    } else {
                        REGULAR_PAIR
                    }
                };

                nc::attron(nc::COLOR_PAIR(pair));
                nc::mv(current_index as i32, 0);
                nc::addstr("hii");
                nc::attr_off(nc::COLOR_PAIR(pair));
            }

            nc::refresh();

            match nc::getch() as u8 as char {
                'j' => {
                    cursor_position += 1;
                }
                'k' => {
                    cursor_position -= 1;
                }
                'q' => {
                    self.quit = true;
                }
                _ => {}
            }
        }

        nc::endwin();
    }
}
