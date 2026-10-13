use crate::{asks::Asks, bids::Bids, level::Level, order::Order, order_id::OrderId, trade::Trade};

#[derive(Debug)]
pub struct OrderBook {
    id_count: OrderId,
    bids: Bids,
    asks: Asks,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            id_count: OrderId::new(0),
            bids: Bids::new(),
            asks: Asks::new(),
        }
    }

    pub fn add_ask(&mut self, price: u32, quantity: u32) -> Trade {
        let id = self.new_order_id();
        let trade = Trade::new(id);

        todo!()
    }

    pub fn add_bid(&mut self, price: u32, quantity: u32) -> Trade {
        let order = Order::new(self.new_order_id(), price, quantity);
        todo!()
    }

    fn new_order_id(&mut self) -> OrderId {
        let id = self.id_count;
        self.id_count = self.id_count.next();
        id
    }
}
