use std::io::*;
pub fn take_input(message: &str) -> String {
    println!("{}", message);
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Nothing provided");
    input
}
