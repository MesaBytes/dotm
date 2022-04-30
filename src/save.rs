use crate::StructDotfile;

pub fn save(path: &String, dotfiles: &Vec<StructDotfile>) -> Result<(), std::io::Error> {
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
