use nc::{getmaxyx, COLOR_PAIR};
use ncurses as nc;

const REGULAR_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;

pub struct Ui {
    max_rows: i32,
    max_columns: i32,
    cursor_position: usize,
    max_menu_rows: usize,
    quit: bool,
}

// int i = 2, height, width;
// WINDOW *new;

// initscr();
// getmaxyx(stdscr, height, width);
// new = newwin(height - 2, width - 2, 1, 1);

// scrollok(new,TRUE);

// while(1)
// {
//     wprintw(new, "%d - lots and lots of lines flowing down the terminal\n", i);
//     ++i;
//     wrefresh(new);
// }

impl Ui {
    pub fn new() -> Ui {
        Ui {
            max_rows: 0,
            max_columns: 0,
            cursor_position: 0,
            max_menu_rows: 0,
            quit: false,
        }
    }

    pub fn init(&mut self) {
        let screen = nc::initscr();

        nc::start_color();
        nc::init_pair(REGULAR_PAIR, nc::COLOR_CYAN, nc::COLOR_BLACK);
        nc::init_pair(HIGHLIGHT_PAIR, nc::COLOR_BLACK, nc::COLOR_WHITE);

        nc::noecho();
        nc::cbreak();
        // nc::raw();
        nc::curs_set(nc::CURSOR_VISIBILITY::CURSOR_INVISIBLE);

        getmaxyx(screen, &mut self.max_rows, &mut self.max_columns);

        let window = nc::newwin(self.max_rows - 2, self.max_columns - 2, 1, 1);

        nc::scrollok(window, true);

        let mut test_items: Vec<i32> = Vec::new();

        for i in 0..100 {
            test_items.push(i);
        }

        self.max_menu_rows = test_items.len() - 1;
        let mut last_index: i32 = 0;
        let mut page_number: i32 = 0;

        while !self.quit {
            nc::refresh();
            for (current_index, item) in test_items.iter().enumerate() {
                // if current_index as i32 == self.max_rows {
                //     last_index = current_index as i32;
                //     page_number += 1;
                //     break;
                // }

                let pair = {
                    if self.cursor_position == current_index {
                        HIGHLIGHT_PAIR
                    } else {
                        REGULAR_PAIR
                    }
                };

                nc::attron(nc::COLOR_PAIR(pair));
                nc::wmove(window, current_index as i32, 0);
                nc::waddstr(window, &format!("{}\n", item.to_string()));
                nc::attr_off(nc::COLOR_PAIR(pair));
            }

            nc::wrefresh(window);

            match nc::getch() as u8 as char {
                'j' => self.move_cursor(Directions::Down),
                'k' => self.move_cursor(Directions::Up),
                'q' => self.quit = true,
                _ => {}
            }
        }
        nc::delwin(window);
        nc::endwin();
    }

    fn move_cursor(&mut self, direction: Directions) {
        match &direction {
            Directions::Up => {
                if self.cursor_position != 0 {
                    self.cursor_position -= 1;
                }
            }
            Directions::Down => {
                if self.cursor_position < self.max_menu_rows
                    || self.max_rows <= self.max_menu_rows as i32
                {
                    self.cursor_position += 1;
                }
            }
        }
    }
}

enum Directions {
    Up,
    Down,
}
