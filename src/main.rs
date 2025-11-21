fn main() {
    output();
}

fn output() {
    println!("{}@{}", user_name(), host());
}

fn user_name() -> String {
    "ssiyad".to_string()
}

fn host() -> String {
    "baghdad".to_string()
}
