use crate::order_id::OrderId;

#[derive(Debug)]
pub struct Order {
    pub id: OrderId,
    pub price: u32,
    pub quantity: u32,
}

impl Order {
    pub fn new(id: OrderId, price: u32, quantity: u32) -> Self {
        Self {
            id,
            price,
            quantity,
        }
    }
}
