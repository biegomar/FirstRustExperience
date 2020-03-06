use std::rc::Rc;

pub fn loops() {
    let name = Rc::new(String::from("Marc"));
    let nachname = Rc::new(String::from("Biegota"));

    println!("Hier kommt ein loop.");
    for _ in 0..3 {
        println!(
            "{:?}",
            Familienname {
                vorname: name.clone(),
                name: nachname.clone(),
            }
        );
    }

    let familie = Familienname {
        vorname: Rc::new(String::from("Klaus")),
        name: Rc::new(String::from("Meyer")),
    };

    println!("{}", familie.display_name());
}

#[derive(Debug)]
struct Familienname {
    name: Rc<String>,
    vorname: Rc<String>,
}

impl Familienname {
    fn display_name(self) -> String {
        format!("{} {}", self.vorname, self.name)
    }
}
