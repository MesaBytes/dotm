// TODO: write add command
// TODO: write remove command
// TODO: write backup command
// TODO: write list command
// TODO: write empty-list command
// TODO: write version command
// TODO: write help command
// if no dotfiles.json is found create one in home dir

use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let command = &args[0];

    if command == "add" {
        let source = &args[1];
        let dest = &args[2];

        println!("source {}, dest {}", source, dest);
    }

    println!("{:?}", args);
}
