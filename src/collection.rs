use std::collections::HashMap;

pub struct Collections {}

impl Collections {
    pub fn array(self: Collections) {
        let numbers: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        println!("Nummer: {}", numbers[6]);
    }

    pub fn tuples(self: Collections) {
        let num_and_str: (u8, &str) = (20, "Toll");
        println!("{:?}", num_and_str);
        println!("{}", num_and_str.0);
    }

    pub fn tuples_deconstruct(self: Collections) {
        let num_and_str: (u8, &str) = (20, "Toll");
        let (num, string) = num_and_str;
        println!("{}", string);
    }

    pub fn vectors(self: Collections) {
        let mut numbers_vec: Vec<u8> = Vec::new();
        numbers_vec.push(10);
        numbers_vec.push(2);

        println!("{}", numbers_vec[0]);
    }

    pub fn hashmap(self: Collections) {
        let mut shopping_cart = HashMap::new();
        shopping_cart.insert("iPhone X", 1);
        shopping_cart.insert("power cord", 1);
        shopping_cart.insert("Schutzhülle", 2);
        println!("Warenkorb");

        for (k, v) in shopping_cart {
            println!("{}x {}", v, k);
        }
    }
}
