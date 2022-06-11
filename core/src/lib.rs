/*
core mod

load    dotfiles
save    dotfiles
add     dotfiles
remove  dotfiles

manage config


*/

use colored::Colorize;
use dircpy::CopyBuilder;
use notify_rust::Notification;
use pbr::ProgressBar;

#[derive(Clone)]
pub struct StructDotfile {
    pub source: String,
    pub destination: String,
}

impl std::fmt::Display for StructDotfile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{0: <35} {1}", self.source, self.destination)
    }
}

pub fn save(path: &String, dotfiles: &Vec<StructDotfile>) {
    let mut contents = String::new();

    for dotfile in dotfiles.iter() {
        contents.push_str(&dotfile.source);
        contents.push(':');
        contents.push_str(&dotfile.destination);
        contents.push('\n');
    }
    std::fs::write(path, contents).expect("Failed to save to db file");
}

pub fn load(path: &String, dotfiles: &mut Vec<StructDotfile>) {
    if std::path::Path::new(path).exists() == false {
        std::fs::write(path, "").expect("Failed to create db file");
    }

    let contents = std::fs::read_to_string(path).expect("Failed to read from database");

    for line in contents.lines() {
        let dotfile: Vec<_> = line.split(':').collect();

        dotfiles.push(StructDotfile {
            source: dotfile[0].to_string(),
            destination: dotfile[1].to_string(),
        });
    }
}

pub fn backup(dotfiles: &Vec<StructDotfile>) {
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

            std::fs::create_dir_all(dests.join("/")).expect("Failed to create all dirs");

            std::fs::copy(&dotfile.source, &dotfile.destination).expect("Failed to Copy File");
        } else if std::path::Path::new(&dotfile.source).is_dir() {
            CopyBuilder::new(dotfile.source.to_owned(), dotfile.destination.to_owned())
                .overwrite(true)
                .run()
                .expect("Failed to Copy directory");
        }

        pb.inc();
    }

    Notification::new()
        .summary("dotm")
        .body("Done backing up dotfiles!")
        .show()
        .expect("Failed to send notification");
}
