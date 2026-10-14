#![allow(dead_code, unused_variables)]

pub struct OrderBook;

impl OrderBook {
    pub fn place(&mut self, side: Side, price: Price, qty: Qty) -> Place {
        todo!()
    }

    pub fn cancel(&mut self, id: Id) -> Qty {
        todo!()
    }

    pub fn get(&mut self, id: Id) -> Qty {
        todo!()
    }

    pub fn best_bid(&self) -> (Price, Qty) {
        todo!()
    }

    pub fn best_ask(&self) -> (Price, Qty) {
        todo!()
    }

    pub fn spread(&self) -> Price {
        todo!()
    }

    pub fn mid_price(&self) -> Price {
        todo!()
    }

    pub fn depth(&self, side: Side, levels: i32) -> Vec<(Price, Qty)> {
        todo!()
    }
}

pub struct Place {
    pub id: Id,
    pub fills: Vec<Fill>,
    pub remaining: Qty,
}

pub struct Fill {
    pub maker_id: Id,
    pub price: Price,
    pub qty: Qty,
}

pub enum Side {
    Ask,
    Bid,
}

pub struct Id(pub u64);
pub struct Price(pub u64);
pub struct Qty(pub u64);
