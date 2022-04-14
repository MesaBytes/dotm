// TODO: write add command
// TODO: write remove command
// TODO: write backup command
// TODO: write list command
// TODO: write empty-list command
// TODO: write version command
// TODO: write help command
// if no dotfiles.json is found create one in home dir

extern crate ncurses;
use ncurses::*;

const REGULAR_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;

fn main() {
    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    start_color();
    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGHLIGHT_PAIR, COLOR_BLACK, COLOR_WHITE);

    let mut quit = false;
    let mut paths = vec![
        "1226::/home/senpai/.books.db::thome",
        "2236::/home/senpai/.config/dotm/::.config",
        "3",
        "4",
        "5",
        "6",
    ];
    let mut curr_idx: usize = 1;

    while !quit {
        for (index, path) in paths.iter().enumerate() {
            let pair = {
                if curr_idx == index {
                    HIGHLIGHT_PAIR
                } else {
                    REGULAR_PAIR
                }
            };
            attron(COLOR_PAIR(pair));
            mv(index as i32, 0);
            addstr(*path);
            attr_off(COLOR_PAIR(pair));
        }

        refresh();

        let key = getch();

        match key as u8 as char {
            'q' => quit = true,
            'k' => {
                if curr_idx == 0 {
                    // Move cursor to the end of the list
                    curr_idx = paths.len() - 1;
                } else {
                    // Move up
                    curr_idx -= 1;
                }
            }
            'j' => {
                if curr_idx == paths.len() - 1 {
                    // Move cursor to the start of the list
                    curr_idx = 0;
                } else {
                    // Move down
                    curr_idx += 1;
                }
            }
            _ => {}
        }
    }

    endwin();
}
