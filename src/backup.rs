use crate::StructDotfile;
use colored::Colorize;
use dircpy::CopyBuilder;
use notify_rust::Notification;
use pbr::ProgressBar;

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
                .overwrite_if_newer(true)
                .overwrite_if_size_differs(true)
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
