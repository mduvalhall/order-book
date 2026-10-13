use std::collections::{BTreeMap, VecDeque};

use crate::order::{Ask, Bid, Buy, Id, Price, Quantity, Sell};

#[derive(Debug)]
pub struct Book {
    current_id: Id,
    asks: BTreeMap<Price, VecDeque<Ask>>,
    bids: BTreeMap<Price, VecDeque<Bid>>,
}

impl Book {
    pub fn new() -> Self {
        Self {
            current_id: Id::new(0),
            asks: BTreeMap::new(),
            bids: BTreeMap::new(),
        }
    }

    pub fn add_ask(&mut self, price: Price, quantity: Quantity) -> Sell {
        todo!()
    }

    pub fn add_bid(&mut self, price: Price, quantity: Quantity) -> Buy {
        todo!()
    }
}
