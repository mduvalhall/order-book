use crate::order::Order;

pub struct Trade {
    id: u32,
    trades: Vec<Order>,
}

impl Trade {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            trades: Vec::new(),
        }
    }

    pub fn add(&mut self, order: Order) {
        self.trades.push(order);
    }
}
