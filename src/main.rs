mod datos_primitivos; 
mod structs; 
mod implements; 


fn main() {
    datos_primitivos::main();
    let user1: structs::USER = structs::create_user(String::from("vitalik"), 1);
    println!("{} {}", user1.name, user1.age);
    implements::main();
}
