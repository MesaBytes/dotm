use crate::StructDotfile;

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
