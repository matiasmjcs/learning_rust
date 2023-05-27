pub struct USER {
        pub name: String,
        pub age: u16,
    }
pub fn main_structs(_name: String, _age: u16) -> USER {
    let user1 = USER{
        name: _name,
        age: _age,
    };
    return user1;
}