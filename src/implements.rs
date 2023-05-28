use crate::structs as structs;

impl structs::USER {
    pub fn print_info(&self) {
        println!("Hello, my name is {} and I am {} years old.", &self.name, &self.age)
    }
}

pub fn main() {
    let user1 = structs::create_user(String::from("vitalik"), 1);
    user1.print_info();
}