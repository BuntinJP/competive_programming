fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let out = s
        .chars()
        .map(|c| {
            if c.is_lowercase() {
                c.to_uppercase().to_string()
            } else {
                c.to_lowercase().to_string()
            }
        })
        .collect::<String>();
    print!("{}", out);
}
