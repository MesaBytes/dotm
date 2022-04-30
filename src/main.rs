mod args;
mod config;
mod input;
extern crate pbr;

use args::*;
use clap::Parser;
use colored::Colorize;
use dircpy::CopyBuilder;
use input::input;
use notify_rust::Notification;
use pbr::ProgressBar;
use std::fmt;
use std::process::exit;
use whoami;
use youchoose;

#[derive(Clone)]
struct StructDotfile {
    source: String,
    destination: String,
}

impl fmt::Display for StructDotfile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "{0: <35} {1}", self.source, self.destination)
    }
}

fn main() {
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
            exit(1);
        }

        config
            .insert(String::from("backup_dir_path"), path.to_string())
            .expect("Failed to get backup_dir_path");

        exit(0);
    }

    load(&dotm_db_path, &mut dotfiles).expect("Failed to load the database");

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
            full_destination.push_str(&destination);

            dotfiles.push(StructDotfile {
                source: source.to_string(),
                destination: full_destination,
            })
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
        Commands::Remove {} => {
            if dotfiles.is_empty() {
                println!("List is empty!");
                exit(0);
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
        }
        Commands::Backup {} => {
            backup(&dotfiles).expect("Failed to backup dotfiles!");
        }
    }

    save(&dotm_db_path, &dotfiles).expect("Failed to save the database");
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
    let count = dotfiles.len();
    let mut pb = ProgressBar::new(count as u64);
    pb.format("[=> ]");

    for i in 0..count {
        let dotfile = &dotfiles[i];
        let source_split: Vec<_> = dotfile.source.split("/").collect();
        let file = source_split[source_split.len() - 1];

        pb.message(&format!("{} ", &file.bright_green()));

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

        pb.inc();
    }

    Notification::new()
        .summary("dotm")
        .body("Done backing up dotfiles!")
        .show()
        .expect("Failed to send notification");

    Ok(())
}
