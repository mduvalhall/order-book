use std::collections::BinaryHeap;

use crate::level::Level;

#[derive(Debug)]
pub struct Bids(BinaryHeap<Level>);

impl Bids {
    pub fn new() -> Self {
        Self(BinaryHeap::new())
    }
}
