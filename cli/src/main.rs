mod args;
mod input;

use core::{dotm::Dotfile, dotm::Dotm, config::Config};
use args::*;
use colored::Colorize;
use clap::Parser;
use input::input;
use std::process::exit;
use whoami;

fn main() {
    let db_path = format!("/home/{}/.config/dotm/dotm.db", whoami::username());
    let config_path = format!("/home/{}/.config/dotm/dotm.conf", whoami::username());

    let mut config = Config::new(&config_path);
    let backup_path = config.get("backup_dir_path").to_string();

    let mut dotm = Dotm::new(db_path);

    let dotfiles = dotm.load().clone();

    if backup_path.is_empty() {
        println!("No backup path is found!");

        let path = input("Enter backup directory: ");

        if std::path::Path::new(&path).exists() == false {
            println!("'{}' is invalid path!", path);
            exit(1);
        }

        config.insert(String::from("backup_dir_path"), path.to_string());
        exit(0);
    }

    

    let cli = DotmArgs::parse();

    match &cli.command {
        Commands::Add {
            source,
            destination,
        } => {
            if !std::path::Path::new(&source).exists() {
                println!("{} does not exists!", source.bright_red());
                exit(1);
            }

            // Check for duplicate entries
            for dotfile in dotfiles.iter() {
                if source.to_string() == dotfile.source {
                    println!("{} already exists!", source.bright_red());
                    exit(1);
                }
            }

            let paths: Vec<_> = source.split("/").collect();
            let file = paths[paths.len() - 1];
            let mut full_destination = backup_path;
            let mut mut_destination = destination.to_string();

            if !destination.ends_with("/") {
                mut_destination.push('/');
            }

            mut_destination.push_str(file);
            full_destination.push_str(&mut_destination);

            dotm.add(Dotfile { source: source.to_string(), destination: full_destination });

            dotm.save();
        }
        Commands::List {} => {
            if dotfiles.is_empty() {
                println!("List is empty!");
                exit(0);
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
        }
        Commands::Remove {
            path
        } => {
            if dotfiles.is_empty() {
                println!("List is empty!");
                exit(0);
            }

            if path == "all" {
                dotm.clear();
            } else {
                let index_element = dotfiles
                    .iter()
                    .position(|x| x.source == *path);

                match index_element {
                    Some(i) => {
                        dotm.remove(i)
                    }
                    None => return
                }
            }

            dotm.save();
        }
        Commands::Backup {} => {
            dotm.backup();
        }
    }

}
