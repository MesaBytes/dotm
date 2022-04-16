//  TODO    handle scrolling (maybe stop addstr if index == term.row and check if curr_idx is not the last index if not the last refresh and only show after term.row if you scroll up refresh again)
//  TODO    Add help option (--help, -h) [list options, list keybinds]
//  TODO    Add a way to add new dotfiles to dotm.db
//  TODO    Add backup option (--backup, -b) with progress bar!

use uuid::Uuid;
use youchoose;

fn main() -> Result<(), std::io::Error> {
    let mut dotfiles = Vec::<String>::new();

    // dotfiles.push(format!(
    //     "{}\t{}\t{}",
    //     Uuid::new_v4().to_string(),
    //     "/home/senpai/books.db",
    //     "home/"
    // ));

    // dotfiles.push(format!(
    //     "{}\t{}\t{}",
    //     Uuid::new_v4().to_string(),
    //     "/home/senpai/.bashrc",
    //     "home/"
    // ));

    load(&mut dotfiles)?;

    // let mut source = String::new();
    // input("source: ", &mut source)?;
    // let mut destination = String::new();
    // input("destination: ", &mut destination)?;

    // let mut menu = youchoose::Menu::new(dotfiles.iter())
    //     .add_up_key('k' as i32)
    //     .add_down_key('j' as i32);

    // let choice = menu.show();

    save(&dotfiles)?;
    Ok(())
}

fn input(message: &str, input_string: &mut String) -> Result<(), std::io::Error> {
    println!("{}", message);
    std::io::stdin().read_line(input_string)?;
    Ok(())
}

fn load(dotfiles: &mut Vec<String>) -> Result<(), std::io::Error> {
    if std::path::Path::new("dotm.db").exists() == false {
        std::fs::write("dotm.db", "")?;
    }

    let contents = std::fs::read_to_string("dotm.db")?;

    for line in contents.lines() {
        dotfiles.push(line.to_string());
    }

    Ok(())
}

fn save(dotfiles: &Vec<String>) -> Result<(), std::io::Error> {
    //  TODO    read line and spit with \t into 3

    let mut contents = String::new();

    for dotfile in dotfiles.iter() {
        let dotfile: Vec<_> = dotfile.split('\t').collect();

        contents.push_str(&dotfile[0]);
        contents.push('\t');
        contents.push_str(&dotfile[1]);
        contents.push('\t');
        contents.push_str(&dotfile[2]);
        contents.push('\n');
    }
    std::fs::write("dotm.db", contents)?;

    Ok(())
}

// fn backup(dotfiles: &Vec<String>) -> Result<(), std::io::Error> {
//     if std::path::Path::new("dotm.db").exists() == false {
//         std::fs::write("dotm.db", "")?;
//     }

//     let contents = std::fs::read_to_string("dotm.db")?;

//     for line in contents.lines() {
//         let dotfile: Vec<_> = line.split('\t').collect();

//         let id = dotfile[0].to_string();
//         let source = dotfile[1].to_string();
//         let destination = dotfile[2].to_string();
//     }

//     Ok(())
// }
