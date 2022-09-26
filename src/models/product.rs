#[derive(Debug)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub price: i32,
}

impl Product {
    fn new(id: i32, name: &str, price: i32) -> Self {
        Self {
            id,
            name: name.to_string(),
            price
        }
    }
}

impl Default for Product {
    fn default() -> Self {
        Self {
            id: -1,
            name: String::new(),
            price: 0
        }
    }
}