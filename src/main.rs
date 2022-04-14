// TODO: write add command
// TODO: write remove command
// TODO: write backup command
// TODO: write list command
// TODO: write empty-list command
// TODO: write version command
// TODO: write help command
// if no dotfiles.json is found create one in home dir

extern crate ncurses;
use ncurses as nc;
use uuid::Uuid;

const REGULAR_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;

struct Dotfile {
    id: String,
    source: String,
    destination: String,
}

fn save(dotfiles: &Vec<Dotfile>) {
    let mut contents = String::new();

    for dotfile in dotfiles.iter() {
        contents.push_str(&dotfile.id);
        contents.push(':');
        contents.push_str(&dotfile.source);
        contents.push(':');
        contents.push_str(&dotfile.destination);
        contents.push('\n');
    }
    std::fs::write("dotm.db", contents);
}

fn main() {
    nc::initscr();
    nc::noecho();
    nc::curs_set(nc::CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    nc::start_color();
    nc::init_pair(REGULAR_PAIR, nc::COLOR_WHITE, nc::COLOR_BLACK);
    nc::init_pair(HIGHLIGHT_PAIR, nc::COLOR_BLACK, nc::COLOR_WHITE);

    let mut quit = false;
    let mut dotfiles: Vec<Dotfile> = Vec::new();
    let mut curr_idx: usize = 0;

    let uuid = Uuid::new_v4();

    dotfiles.push(Dotfile {
        id: uuid.to_string(),
        source: String::from("/home/senpai/books.db"),
        destination: String::from("home/"),
    });

    dotfiles.push(Dotfile {
        id: uuid.to_string(),
        source: String::from("/home/senpai/.bashrc"),
        destination: String::from("home/"),
    });

    while !quit {
        for (index, dotfile) in dotfiles.iter().enumerate() {
            let pair = {
                if curr_idx == index {
                    HIGHLIGHT_PAIR
                } else {
                    REGULAR_PAIR
                }
            };
            let format = std::format!(
                "{}\t{}\t{}",
                dotfile.id,
                dotfile.source,
                dotfile.destination
            );

            nc::attron(nc::COLOR_PAIR(pair));
            nc::mv(index as i32, 0);
            nc::addstr(&format);
            nc::attr_off(nc::COLOR_PAIR(pair));
        }

        nc::refresh();

        let key = nc::getch();

        match key as u8 as char {
            'q' => quit = true,
            'k' => {
                if curr_idx == 0 {
                    // Move cursor to the end of the list
                    curr_idx = dotfiles.len() - 1;
                } else {
                    // Move up
                    curr_idx -= 1;
                }
            }
            'j' => {
                if curr_idx == dotfiles.len() - 1 {
                    // Move cursor to the start of the list
                    curr_idx = 0;
                } else {
                    // Move down
                    curr_idx += 1;
                }
            }
            'd' => {
                dotfiles.remove(curr_idx);
                save(&dotfiles);
                nc::clear();
            }
            _ => {}
        }
    }

    nc::endwin();
}
