use crate::{order::Order, order_id::OrderId};

pub struct Trade {
    id: OrderId,
    trades: Vec<Order>,
}

impl Trade {
    pub fn new(id: OrderId) -> Self {
        Self {
            id,
            trades: Vec::new(),
        }
    }

    pub fn add(&mut self, order: Order) {
        self.trades.push(order);
    }
}
