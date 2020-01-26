use std::env;

fn main() {
    greetings();
}

fn greetings() {
    let first_name = env::args().skip(1).next();

    match first_name {
        Some(n) => println!("Hello {}!", n),
        None => panic!("Hast Du den Vornamen vergessen?"),
    }
}
