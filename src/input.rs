pub fn input(message: &str) -> String {
    println!("{}", message);
    let mut ret = String::new();

    std::io::stdin()
        .read_line(&mut ret)
        .expect("Failed to read from stdin");
    ret
}
