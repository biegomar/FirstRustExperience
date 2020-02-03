use crate::behaviour::SalesDiscount;

pub struct Customers {
    pub name: String,
    pub strasse: String,
    pub plz: String,
    pub ort: String,
}

impl SalesDiscount for Customers {
    fn discount(&self) -> u8 {
        35
    }
}
