pub struct Person {
    pub vorname: String,
    pub nachname: String,
    pub alter: u8,
}

impl Person {
    pub fn print_diplay_name(&self) {
        println!(
            "Person: {} {}, {} Jahre.",
            self.vorname, self.nachname, self.alter
        );
    }

    pub fn get_display_name(&self) -> String {
        format!(
            "Person: {} {}, {} Jahre.",
            self.vorname, self.nachname, self.alter
        )
    }

    pub fn print_only_name(&self) {
        println!("Person: {} {}.", self.vorname, self.nachname);
    }
}
