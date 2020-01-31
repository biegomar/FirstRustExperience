mod customer;

use customer::Customers;
use std::collections::HashMap;
use std::env;

struct Color(u8, u8, u8);

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

    using_struct();
    using_struct_deconstruct();

    c_style_struct();
    use_struct_implementation();

    use_customer_from_module();
    use_struct_implementation_from_module();

    array();
    tuples();
    tuples_deconstruct();
    vectors();
    hashmap();
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

fn using_struct() {
    let white = Color(255, 255, 255);

    let red = white.0;
    let green = white.1;
    let blue = white.2;

    println!("Rot: {}, Grün: {}, Blau: {}", red, green, blue);
}

fn using_struct_deconstruct() {
    let white = Color(255, 255, 255);
    let Color(r, g, b) = white;

    println!("Rot: {}, Grün: {}, Blau: {}", r, g, b);

    let orange = Color(255, 165, 0);
    let Color(r, _, b) = orange;

    println!("Rot: {}, Grün: {}, Blau: {}", r, g, b);
}

struct Person {
    vorname: String,
    nachname: String,
    alter: u8,
}

impl Person {
    fn print_diplay_name(&self) {
        println!(
            "Person: {} {}, {} Jahre.",
            self.vorname, self.nachname, self.alter
        );
    }

    fn get_display_name(&self) -> String {
        format!(
            "Person: {} {}, {} Jahre.",
            self.vorname, self.nachname, self.alter
        )
    }
}

fn c_style_struct() {
    let person = Person {
        vorname: "Marc".to_string(),
        nachname: "Biegota".to_string(),
        alter: 47,
    };

    println!(
        "Person: {} {}, {} Jahre.",
        person.vorname, person.nachname, person.alter
    );
}

fn use_struct_implementation() {
    let person = Person {
        vorname: "Marc".to_string(),
        nachname: "Biegota".to_string(),
        alter: 47,
    };

    person.print_diplay_name();
    println!("{}", person.get_display_name());

    let display_name = Person::get_display_name(&person);
    println!("{}", display_name);
}

fn use_customer_from_module() {
    let cust = Customers {
        name: "Kunde".to_string(),
        strasse: "Testweg 12".to_string(),
        plz: "58300".to_string(),
        ort: "Wetter".to_string(),
    };

    println!("Kunde: {}", cust.name);
}

fn use_struct_implementation_from_module() {
    let person = Person {
        vorname: "Marc".to_string(),
        nachname: "Biegota".to_string(),
        alter: 47,
    };

    person.print_only_name();
}

fn array() {
    let numbers: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    println!("Nummer: {}", numbers[6]);
}

fn tuples() {
    let num_and_str: (u8, &str) = (20, "Toll");
    println!("{:?}", num_and_str);
    println!("{}", num_and_str.0);
}

fn tuples_deconstruct() {
    let num_and_str: (u8, &str) = (20, "Toll");
    let (num, string) = num_and_str;
    println!("{}", string);
}

fn vectors() {
    let mut numbers_vec: Vec<u8> = Vec::new();
    numbers_vec.push(10);
    numbers_vec.push(2);

    println!("{}", numbers_vec[0]);
}

fn hashmap() {
    let mut shopping_cart = HashMap::new();
    shopping_cart.insert("iPhone X", 1);
    shopping_cart.insert("power cord", 1);
    shopping_cart.insert("Schutzhülle", 2);
    println!("Warenkorb");

    for (k, v) in shopping_cart {
        println!("{}x {}", v, k);
    }
}
