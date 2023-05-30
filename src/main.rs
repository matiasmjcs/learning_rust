mod datos_primitivos;
mod implements;
mod structs;

fn main() {
    datos_primitivos::main();
    implements::main();
    let user1: structs::USER = structs::create_user(String::from("vitalik"), 1);
    user1.print_info();
}
