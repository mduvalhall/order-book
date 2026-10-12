#[derive(Debug)]
pub struct Order {
    pub id: u64,
    pub price: u32,
    pub quantity: u32,
}

impl Order {
    pub fn new(id: u64, price: u32, quantity: u32) -> Self {
        Self {
            id,
            price,
            quantity,
        }
    }
}
