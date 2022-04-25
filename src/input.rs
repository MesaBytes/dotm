use std::io::Write;

pub fn input(message: &str) -> String {
    print!("{}", message);
    let mut ret = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut ret)
        .expect("Failed to read from stdin");
    return ret.trim().to_string();
}
