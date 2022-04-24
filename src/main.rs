//  TODO    Add backup option (--backup, -b) with progress bar!
//  TODO    Better printing for list command

mod config;
mod input;
use colored::Colorize;
use std::{env, process};
use whoami;
use youchoose;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut dotfiles = Vec::<String>::new();
    let dotm_db_path = format!("/home/{}/.config/dotm/dotm.db", whoami::username());
    let dotm_config_path = format!("/home/{}/.config/dotm/dotm.conf", whoami::username());

    let mut config =
        config::Config::new(&dotm_config_path).expect("Failed initialization the database!");
    let backup_path = config.get("backup_dir_path");

    if backup_path.is_empty() {
        println!("No backup path is found!");

        let path = input::input("Enter backup directory: ");

        if std::path::Path::new(&path.trim()).exists() == false {
            println!("'{}' is invalid path!", path.trim());
            process::exit(1);
        }

        config.insert(String::from("backup_dir_path"), path.trim().to_string())?;

        process::exit(0);
    }

    load(&dotm_db_path, &mut dotfiles)?;

    if args.len() != 0 {
        if args[0] == "add" || args[0] == "a" {
            let source = &args[1];
            let destination = &args[2];

            if std::path::Path::new(&source).exists() == false {
                println!("[Error]\t{} does not exists!", source);
                process::exit(1);
            }

            dotfiles.push(format!("{}\t{}", source, destination))
        } else if args[0] == "remove" || args[0] == "r" {
            let menu_list = dotfiles.clone();
            let mut menu = youchoose::Menu::new(menu_list.iter())
                .add_up_key('k' as i32)
                .add_down_key('j' as i32);

            let choice = menu.show();

            if choice.len() != 0 {
                dotfiles.remove(choice[0]);
            }
        } else if args[0] == "list" || args[0] == "l" {
            for dotfile in dotfiles.iter() {
                let dotfile: Vec<_> = dotfile.split('\t').collect();

                print!("{}", dotfile[0].bright_green());
                print!("\t");
                print!("{}\n", dotfile[1].bright_yellow());
            }
        }
    }
    if args.len() == 0 || args[0] == "--help" || args[0] == "-h" {
        println!(
            "{} dotm [options] [command]
\t\tDotfiles manager
{}
    add,        a    Add new path, [source, destination]
    remove,     r    Remove path
    list,       l    List dotfiles

{}
    --help,     -h   Print this message",
            format!(" Usage ").on_bright_green().black().bold(),
            format!(" Command ").on_bright_green().black().bold(),
            format!(" Options ").on_bright_green().black().bold()
        )
    }

    save(&dotm_db_path, &dotfiles)?;
    Ok(())
}

fn load(path: &String, dotfiles: &mut Vec<String>) -> Result<(), std::io::Error> {
    if std::path::Path::new(path).exists() == false {
        std::fs::write(path, "")?;
    }

    let contents = std::fs::read_to_string(path)?;

    for line in contents.lines() {
        let dotfile: Vec<_> = line.split(':').collect();

        dotfiles.push(format!("{}\t{}", &dotfile[0], &dotfile[1]));
    }

    Ok(())
}

fn save(path: &String, dotfiles: &Vec<String>) -> Result<(), std::io::Error> {
    let mut contents = String::new();

    for dotfile in dotfiles.iter() {
        let dotfile: Vec<_> = dotfile.split('\t').collect();

        contents.push_str(&dotfile[0]);
        contents.push(':');
        contents.push_str(&dotfile[1]);
        contents.push('\n');
    }
    std::fs::write(path, contents)?;

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
