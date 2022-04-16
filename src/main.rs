//  TODO    Add backup option (--backup, -b) with progress bar!

use std::{env, process::exit};
use youchoose;

fn main() -> Result<(), std::io::Error> {
    let arg: Vec<String> = env::args().skip(1).collect();
    let mut dotfiles = Vec::<String>::new();

    load(&mut dotfiles)?;

    if arg[0] == "--add" || arg[0] == "-a" {
        let source: String = input(&"source: ");
        let destination: String = input(&"destination: ");

        if std::path::Path::new(&source).exists() == false {
            println!("[Error]\t{} does not exists!", source);
            exit(1);
        }

        dotfiles.push(format!("{}\t{}", source, destination))
    } else if arg[0] == "--help" || arg[0] == "-h" {
        println!(
            "--- dotm help ---

Options:
    --add,    -a    Add new path
    --remove, -r    Remove path
    --help,   -h    Print this message"
        )
    }

    // let mut menu = youchoose::Menu::new(dotfiles.iter())
    //     .add_up_key('k' as i32)
    //     .add_down_key('j' as i32);

    // let choice = menu.show();

    save(&dotfiles)?;
    Ok(())
}

fn input(message: &'_ impl std::fmt::Display) -> String {
    // TODO     inline message and stdin
    println!("{}", message);
    let mut ret = String::new();
    std::io::stdin()
        .read_line(&mut ret)
        .expect("Failed to read from stdin");
    ret.trim().parse().expect("Failed to parse")
}

fn load(dotfiles: &mut Vec<String>) -> Result<(), std::io::Error> {
    if std::path::Path::new("dotm.db").exists() == false {
        std::fs::write("dotm.db", "")?;
    }

    let contents = std::fs::read_to_string("dotm.db")?;

    for line in contents.lines() {
        let dotfile: Vec<_> = line.split(':').collect();

        dotfiles.push(format!("{}\t{}", &dotfile[0], &dotfile[1]));
    }

    Ok(())
}

fn save(dotfiles: &Vec<String>) -> Result<(), std::io::Error> {
    let mut contents = String::new();

    for dotfile in dotfiles.iter() {
        let dotfile: Vec<_> = dotfile.split('\t').collect();

        contents.push_str(&dotfile[0]);
        contents.push(':');
        contents.push_str(&dotfile[1]);
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
