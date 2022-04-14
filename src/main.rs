//  TODO    Add scrolling
//  TODO    Add help option (--help, -h) [list options, list keybinds]
//  TODO    Add a way to add new dotfiles to dotm.db
//  TODO    Add backup option (--backup, -b) with progress bar!

extern crate ncurses;
extern crate termsize;
use ncurses as nc;
use uuid::Uuid;

const REGULAR_PAIR: i16 = 0;
const HIGHLIGHT_PAIR: i16 = 1;

struct Dotfile {
    id: String,
    source: String,
    destination: String,
}

fn load(dotfiles: &mut Vec<Dotfile>) -> Result<(), std::io::Error> {
    if std::path::Path::new("dotm.db").exists() == false {
        std::fs::write("dotm.db", "")?;
    }

    let contents = std::fs::read_to_string("dotm.db")?;

    for line in contents.lines() {
        let dotfile: Vec<_> = line.split('\t').collect();

        // (id, source, destination)
        dotfiles.push(Dotfile {
            id: dotfile[0].to_string(),
            source: dotfile[1].to_string(),
            destination: dotfile[2].to_string(),
        });
    }

    Ok(())
}

fn save(dotfiles: &Vec<Dotfile>) -> Result<(), std::io::Error> {
    let mut contents = String::new();

    for dotfile in dotfiles.iter() {
        contents.push_str(&dotfile.id);
        contents.push('\t');
        contents.push_str(&dotfile.source);
        contents.push('\t');
        contents.push_str(&dotfile.destination);
        contents.push('\n');
    }
    std::fs::write("dotm.db", contents)?;

    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let term = termsize::get().unwrap();

    nc::initscr();
    nc::noecho();
    nc::curs_set(nc::CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    nc::start_color();
    nc::init_pair(REGULAR_PAIR, nc::COLOR_WHITE, nc::COLOR_BLACK);
    nc::init_pair(HIGHLIGHT_PAIR, nc::COLOR_BLACK, nc::COLOR_WHITE);

    let mut quit = false;
    let mut dotfiles = Vec::<Dotfile>::new();
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

    load(&mut dotfiles)?;

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

        if dotfiles.len() == 0 {
            nc::mv(0, 0);
            nc::addstr("List empty, --help for help");
        }
        nc::mv(term.rows as i32 - 1, 0);
        nc::addstr("q: quit\td: delete");

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
                if dotfiles.len() > 0 {
                    dotfiles.remove(curr_idx);
                    nc::clear();

                    // Move cursor
                    if curr_idx != 0 {
                        curr_idx = curr_idx - 1;
                    }
                }
            }
            _ => {}
        }
    }

    save(&dotfiles)?;
    nc::endwin();
    Ok(())
}
