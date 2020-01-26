use std::env;

fn main() {
    greetings();

    println!("Summe von 10 und 21 ist {}.", add(10, 21));
    println!(
        "Summe von 10 und 21 ist auch hier {}.",
        add_with_closure(10, 21)
    );

    if_statement("Test");
    if_statement("Hallo Welt!");

    println!("{}", if_expression("Test"));
    println!("{}", if_expression("Hallo Welt!"));

    loop_for_range();
    loop_for_range_inclusive_right_border();

    while_with_mutable();

    loop_loop_mutable();
}

fn greetings() {
    let first_name = env::args().skip(1).next();

    match first_name {
        Some(n) => println!("Hello {}!", n),
        //None => panic!("Hast Du den Vornamen vergessen?"),
        None => println!("Hast Du den Vornamen vergessen?"),
    }
}

fn add(summand_a: i32, summand_b: i32) -> i32 {
    summand_a + summand_b
}

fn add_with_closure(summand_a: i32, summand_b: i32) -> i32 {
    let addy_closure = |a: i32, b: i32| -> i32 { a + b };

    addy_closure(summand_a, summand_b)
}

fn if_statement(value: &str) {
    if value == "Test" {
        println!("Dies ist ein Test");
    } else {
        println!("{}", value);
    }
}

fn if_expression(value: &str) -> &str {
    let result = if value == "Test" {
        "Dies ist ein Test"
    } else {
        value
    };

    result
}

fn loop_for_range() {
    for counter in 1..5 {
        println!("Wir sind bei Nummer {}", counter);
    }
}

fn loop_for_range_inclusive_right_border() {
    for counter in 1..=5 {
        println!("Wir sind bei Nummer {}", counter);
    }
}

fn while_with_mutable() {
    let mut counter = 5;

    while counter >= 0 {
        println!("Countdown: {}", counter);
        counter -= 1;
    }
}

fn loop_loop_mutable() {
    let mut counter = 5;

    loop {
        if counter < 0 {
            break;
        }
        println!("Countdown with loop: {}", counter);
        counter -= 1;
    }
}
