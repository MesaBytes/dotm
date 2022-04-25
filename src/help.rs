use colored::Colorize;

pub fn help() {
    println!(
        "{} dotm [command] [options]",
        format!(" Usage ").on_bright_green().black().bold()
    );
    println!("\t\tDotfiles manager");
    println!();
    println!("{}", format!(" Command ").on_bright_green().black().bold());
    entry("add", "a", "Add new dotfile", "<src> <dest>");
    entry("remove", "r", "Remove dotfile", "");
    entry("list", "l", "List dotfiles", "");
    entry("backup", "b", "backup Backup dotfiles", "");

    println!();
    println!("{}", format!(" Options ").on_bright_green().black().bold());
    entry("--help", "-h", "Print this message", "");
    entry("--version", "-v", "Print version", "");
}

fn entry(name: &str, alias: &str, description: &str, args: &str) {
    if !args.is_empty() {
        println!(
            "{0}, {1} {2:15}\t{3}",
            alias.bright_yellow(),
            name.bright_yellow(),
            args.bright_green(),
            description.bright_black()
        )
    } else {
        println!(
            "{0}, {1:15}\t{2}",
            alias.bright_yellow(),
            name.bright_yellow(),
            description.bright_black()
        )
    }
}
