use crate::StructDotfile;

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
