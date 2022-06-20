use colored::Colorize;
use dircpy::CopyBuilder;
use notify_rust::Notification;
use pbr::ProgressBar;

#[derive(Clone)]
pub struct Dotfile {
    pub source: String,
    pub destination: String,
}

impl std::fmt::Display for Dotfile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{0: <35} {1}", self.source, self.destination)
    }
}

pub struct Dotm {
    db_path: String,
    dotfiles: Vec::<Dotfile>
}

impl Dotm {
    pub fn new(db_path: String) -> Self {
        if std::path::Path::new(&db_path).exists() == false {
            std::fs::write(&db_path, "").expect("Failed to create db file");
        }

        Self {
            db_path,
            dotfiles: Vec::new()
        }
    }

    /// Add dotfile
    pub fn add(&mut self, dotfile: Dotfile) {
        self.dotfiles.push(dotfile)
    }

    /// Remove dotfile
    pub fn remove(&mut self, index: usize) {
        self.dotfiles.remove(index);
    }

    /// Save dotfiles to dotfiles.json
    pub fn save(&self) {
        let mut contents = String::new();

        for dotfile in self.dotfiles.iter() {
            contents.push_str(&dotfile.source);
            contents.push(':');
            contents.push_str(&dotfile.destination);
            contents.push('\n');
        }
        std::fs::write(&self.db_path, contents).expect("Failed to save to db file");
    }

    pub fn clear(&mut self) {
        self.dotfiles.clear();
    }

    /// Load dotfiles from dotfiles.json to self.dotfiles
    pub fn load(&mut self) -> &Vec<Dotfile> {
        let contents = std::fs::read_to_string(&self.db_path).expect("Failed to read from database");
        
        for line in contents.lines() {
            let dotfile: Vec<_> = line.split(':').collect();

            self.dotfiles.push(Dotfile {
                source: dotfile[0].to_string(),
                destination: dotfile[1].to_string(),
            });
        }

        return &self.dotfiles;
    }

    /// Copy files to backup dir
    pub fn backup(&mut self) {
        let count = self.dotfiles.len();
        let mut pb = ProgressBar::new(count as u64);
        pb.format("[=> ]");

        for i in 0..count {
            let dotfile = &self.dotfiles[i];
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
}
