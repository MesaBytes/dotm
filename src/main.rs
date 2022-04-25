mod config;
mod input;

use colored::Colorize;
use dircpy::CopyBuilder;
use input::input;
use std::{env, process};
use whoami;
use youchoose;

#[derive(Clone)]
struct StructDotfile {
    source: String,
    destination: String,
}

use std::fmt;

impl fmt::Display for StructDotfile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "{0: <35} {1}", self.source, self.destination)
    }
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut dotfiles = Vec::<StructDotfile>::new();
    let dotm_db_path = format!("/home/{}/.config/dotm/dotm.db", whoami::username());
    let dotm_config_path = format!("/home/{}/.config/dotm/dotm.conf", whoami::username());

    let mut config =
        config::Config::new(&dotm_config_path).expect("Failed initialization the database!");
    let backup_path = config.get("backup_dir_path").to_string();

    if backup_path.is_empty() {
        println!("No backup path is found!");

        let path = input("Enter backup directory: ");

        if std::path::Path::new(&path).exists() == false {
            println!("'{}' is invalid path!", path);
            process::exit(1);
        }

        config.insert(String::from("backup_dir_path"), path.to_string())?;

        process::exit(0);
    }

    load(&dotm_db_path, &mut dotfiles)?;

    if args.len() != 0 {
        if args[0] == "add" || args[0] == "a" {
            let source = &args[1];
            let mut destination = args[2].to_string();

            if std::path::Path::new(&source).exists() == false {
                println!("[Error]\t{} does not exists!", source);
                process::exit(1);
            }

            let paths: Vec<_> = source.split("/").collect();
            let file = paths[paths.len() - 1];
            let mut full_destination = backup_path;

            if !destination.ends_with("/") {
                destination.push('/');
            }

            destination.push_str(file);
            full_destination.push_str(&destination);

            dotfiles.push(StructDotfile {
                source: source.to_string(),
                destination: full_destination,
            })
        } else if args[0] == "remove" || args[0] == "r" {
            if dotfiles.is_empty() {
                println!("List is empty!");
                process::exit(0);
            }

            let mut quit = false;
            while quit == false && dotfiles.len() > 0 {
                let menu_list = dotfiles.clone();

                let mut menu = youchoose::Menu::new(menu_list.iter())
                    .add_up_key('k' as i32)
                    .add_down_key('j' as i32);

                let choices = menu.show();

                if choices.len() == 0 {
                    quit = true;
                } else {
                    dotfiles.remove(choices[0]);
                }
            }
        } else if args[0] == "list" || args[0] == "l" {
            if dotfiles.is_empty() {
                println!("List is empty!");
                process::exit(0);
            }

            println!("{0: <35} {1}", "Source", "Destination");
            println!("-----------------------------------------------------");
            for dotfile in dotfiles.iter() {
                println!(
                    "{0: <35} {1}",
                    dotfile.source.bright_green(),
                    dotfile.destination.bright_yellow()
                );
            }
        } else if args[0] == "backup" || args[0] == "b" {
            backup(&dotfiles)?;
        } else if args[0] == "--version" || args[0] == "-v" {
            println!("Version: {}", VERSION.bright_yellow());
        }
    }
    if args.len() == 0 || args[0] == "--help" || args[0] == "-h" {
        println!(
            "{} dotm [options] [command]
\t\tDotfiles manager
{}
    add <s> <d>  a   Add new dotfile
    remove       r   Remove dotfile
    list         l   List dotfiles
    backup       b   Backup dotfiles

{}
    --help,     -h   Print this message
    --version   -v   Print version",
            format!(" Usage ").on_bright_green().black().bold(),
            format!(" Command ").on_bright_green().black().bold(),
            format!(" Options ").on_bright_green().black().bold()
        )
    }

    save(&dotm_db_path, &dotfiles)?;
    Ok(())
}

fn load(path: &String, dotfiles: &mut Vec<StructDotfile>) -> Result<(), std::io::Error> {
    if std::path::Path::new(path).exists() == false {
        std::fs::write(path, "")?;
    }

    let contents = std::fs::read_to_string(path)?;

    for line in contents.lines() {
        let dotfile: Vec<_> = line.split(':').collect();

        dotfiles.push(StructDotfile {
            source: dotfile[0].to_string(),
            destination: dotfile[1].to_string(),
        });
    }

    Ok(())
}

fn save(path: &String, dotfiles: &Vec<StructDotfile>) -> Result<(), std::io::Error> {
    let mut contents = String::new();

    for dotfile in dotfiles.iter() {
        contents.push_str(&dotfile.source);
        contents.push(':');
        contents.push_str(&dotfile.destination);
        contents.push('\n');
    }
    std::fs::write(path, contents)?;

    Ok(())
}

fn backup(dotfiles: &Vec<StructDotfile>) -> Result<(), std::io::Error> {
    // TODO add progress bar
    for dotfile in dotfiles.iter() {
        if std::path::Path::new(&dotfile.source).is_file() {
            // TODO: Find better names :(
            let mut dests: Vec<_> = dotfile.destination.split("/").collect();

            // Remove file remove vector
            dests.pop();

            std::fs::create_dir_all(dests.join("/"))?;

            std::fs::copy(&dotfile.source, &dotfile.destination)?;
        } else if std::path::Path::new(&dotfile.source).is_dir() {
            CopyBuilder::new(dotfile.source.to_owned(), dotfile.destination.to_owned())
                .overwrite_if_newer(true)
                .overwrite_if_size_differs(true)
                .run()?;
        }
    }

    Ok(())
}
