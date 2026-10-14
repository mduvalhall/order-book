use std::collections::{BTreeMap, VecDeque};

use crate::order::{Ask, Bid, Order, Price};

#[derive(Debug)]
pub struct Asks {
    asks: BTreeMap<Price, VecDeque<Order<Ask>>>,
}

impl Asks {
    pub fn new() -> Self {
        Self {
            asks: BTreeMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct Bids {
    bids: BTreeMap<Price, VecDeque<Order<Bid>>>,
}

impl Bids {
    pub fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
        }
    }
}
