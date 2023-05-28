pub struct USER {
        pub name: String,
        pub age: u16,
    }
pub fn create_user(_name: String, _age: u16) -> USER {
    USER{
        name: _name,
        age: _age,
    }
}