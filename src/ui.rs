use ncurses as nc;

const REGULAR_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;

pub struct Ui {
    max_rows: i32,
    max_columns: i32,
    cursor_position: usize,
    max_menu_rows: usize,
    pager: Pager,
    quit: bool,
}

struct Pager {
    page_number: usize,
    max_pages: usize,
    last_list_index: usize,
}

impl Ui {
    pub fn new() -> Ui {
        Ui {
            max_rows: 0,
            max_columns: 0,
            cursor_position: 0,
            max_menu_rows: 0,
            pager: Pager {
                page_number: 1,
                max_pages: 1,
                last_list_index: 0,
            },
            quit: false,
        }
    }

    pub fn init(mut self) {
        let screen = nc::initscr();

        nc::start_color();
        nc::init_pair(REGULAR_PAIR, nc::COLOR_CYAN, nc::COLOR_BLACK);
        nc::init_pair(HIGHLIGHT_PAIR, nc::COLOR_BLACK, nc::COLOR_WHITE);

        nc::noecho();
        nc::cbreak();
        nc::curs_set(nc::CURSOR_VISIBILITY::CURSOR_INVISIBLE);

        nc::getmaxyx(screen, &mut self.max_rows, &mut self.max_columns);

        // Main loop
        while !self.quit {
            nc::refresh();
            self.display();
            self.get_keys();
        }
        nc::endwin();
    }

    fn get_keys(&mut self) {
        match nc::getch() as u8 as char {
            'j' => self.move_cursor(Directions::Down),
            'k' => self.move_cursor(Directions::Up),
            'q' => self.quit = true,
            _ => {}
        }
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

    fn display(&mut self) {
        let mut test_items: Vec<i32> = Vec::new();

        for i in 0..100 {
            test_items.push(i);
        }

        self.max_menu_rows = test_items.len() - 1;

        for current_index in 0..test_items.len() {
            let mut item = test_items[current_index];

            let pair = {
                if self.cursor_position == current_index {
                    HIGHLIGHT_PAIR
                } else {
                    REGULAR_PAIR
                }
            };

            if current_index as i32 == self.max_rows - 3 {
                self.pager.last_list_index = current_index;
                break;
            }

            if self.cursor_position as i32 == self.max_rows - 3 {
                // clear screen and increment page number
                // change last index
                nc::clear();
                self.pager.page_number += 1;
                // nc::attron(nc::COLOR_PAIR(pair));
                // nc::mv(current_index as i32, 0);
                // nc::addstr(&format!("{}", item.to_string()));
                // nc::attr_off(nc::COLOR_PAIR(pair));
            } else {
                nc::attron(nc::COLOR_PAIR(pair));
                nc::mv(current_index as i32, 0);
                nc::addstr(&format!("{}", item.to_string()));
                nc::attr_off(nc::COLOR_PAIR(pair));
            }
        }

        nc::mv(self.max_rows - 1, 0);
        nc::addstr(&format!("page {}", self.pager.page_number));

        nc::mv(self.max_rows - 1, 10);
        nc::addstr(&format!("cursor_position {}", self.cursor_position));

        // nc::mv(self.max_rows - 1, 20);
        // nc::addstr(&format!("page {}", self.));
    }
}

enum Directions {
    Up,
    Down,
}
