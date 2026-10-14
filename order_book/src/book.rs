use crate::{
    order::{Ask, Bid, Id, Price, Quantity, Side},
    orders::{Asks, Bids},
    trade::Trade,
};

#[derive(Debug)]
pub struct Book {
    current_id: Id,
    asks: Asks,
    bids: Bids,
}

impl Book {
    pub fn new() -> Self {
        Self {
            current_id: Id::new(0),
            asks: Asks::new(),
            bids: Bids::new(),
        }
    }

    pub fn add_ask(&mut self, price: Price, quantity: Quantity) -> Trade<Ask> {
        todo!("Not ready to implement")
    }

    pub fn add_bid(&mut self, price: Price, quantity: Quantity) -> Trade<Bid> {
        todo!("Not ready to implement")
    }
}

impl Default for Book {
    fn default() -> Self {
        Self::new()
    }
}
