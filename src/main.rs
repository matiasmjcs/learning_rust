mod variables {
    pub mod main;
}

mod structs {
    pub mod main;
}

fn main() {
    variables::main::main_variables();
    let user1: structs::main::USER = structs::main::main_structs(String::from("cafesito"), 1);
    println!("{} {}", user1.name, user1.age);
}
