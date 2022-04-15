//  TODO    handle scrolling (maybe stop addstr if index == term.row and check if curr_idx is not the last index if not the last refresh and only show after term.row if you scroll up refresh again)
//  TODO    Add help option (--help, -h) [list options, list keybinds]
//  TODO    Add a way to add new dotfiles to dotm.db
//  TODO    Add backup option (--backup, -b) with progress bar!

use uuid::Uuid;
use youchoose;

struct Dotfile {
    id: String,
    source: String,
    destination: String,
}

fn load(dotfiles: &mut Vec<String>) -> Result<(), std::io::Error> {
    if std::path::Path::new("dotm.db").exists() == false {
        std::fs::write("dotm.db", "")?;
    }

    let contents = std::fs::read_to_string("dotm.db")?;

    for line in contents.lines() {
        let dotfile: Vec<_> = line.split('\t').collect();

        // (id, source, destination)
        // dotfiles.push(Dotfile {
        //     id: dotfile[0].to_string(),
        //     source: dotfile[1].to_string(),
        //     destination: dotfile[2].to_string(),
        // });
        dotfiles.push(format!(
            "{}\t{}\t{}",
            dotfile[0].to_string(),
            dotfile[1].to_string(),
            dotfile[2].to_string()
        ));
    }

    Ok(())
}

fn save(dotfiles: &Vec<String>) -> Result<(), std::io::Error> {
    //  TODO    read line and spit with \t into 3

    // let mut contents = String::new();

    // for dotfile in dotfiles.iter() {
    //     contents.push_str(&dotfile.id);
    //     contents.push('\t');
    //     contents.push_str(&dotfile.source);
    //     contents.push('\t');
    //     contents.push_str(&dotfile.destination);
    //     contents.push('\n');
    // }
    // std::fs::write("dotm.db", contents)?;

    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let mut dotfiles = Vec::<String>::new();

    // dotfiles.push(Dotfile {
    //     id: Uuid::new_v4().to_string(),
    //     source: String::from("/home/senpai/books.db"),
    //     destination: String::from("home/"),
    // });

    // dotfiles.push(Dotfile {
    //     id: Uuid::new_v4().to_string(),
    //     source: String::from("/home/senpai/.bashrc"),
    //     destination: String::from("home/"),
    // });

    for i in 0..100 {
        dotfiles.push(format!(
            "{}\t{}\t{}",
            Uuid::new_v4().to_string(),
            i.to_string(),
            i.to_string()
        ));
    }
    // load(&mut dotfiles)?;

    let mut menu = youchoose::Menu::new(dotfiles.iter())
        .add_up_key('k' as i32)
        .add_down_key('j' as i32);
    let choice = menu.show()[0];

    println!("Index of the chosen item: {}", choice);
    println!("item: {}", dotfiles[choice]);

    // save(&dotfiles)?;
    Ok(())
}
