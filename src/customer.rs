use Person;

pub struct Customers {
    pub name: String,
    pub strasse: String,
    pub plz: String,
    pub ort: String,
}

impl Person {
    pub fn print_only_name(&self) {
        println!("Person: {} {}.", self.vorname, self.nachname);
    }
}
