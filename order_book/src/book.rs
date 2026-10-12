use std::{cmp::Reverse, collections::BinaryHeap};

use crate::{level::Level, trade::Trade};

#[derive(Debug)]
pub struct Book {
    id_count: u32,
    bids: BinaryHeap<Level>,
    asks: BinaryHeap<Reverse<Level>>,
}

impl Book {
    pub fn new() -> Self {
        Self {
            id_count: 0,
            bids: BinaryHeap::new(),
            asks: BinaryHeap::new(),
        }
    }

    fn get_next_id(&mut self) -> u32 {
        let id = self.id_count;
        self.id_count += 1;
        id
    }

    fn ask(&mut self, price: u32, quantity: u32) -> Trade {
        todo!()
    }

    fn bid(&mut self, price: u32, quantity: u32) -> Trade {
        todo!()
    }
}
