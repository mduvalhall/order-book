use std::{cmp::Reverse, collections::BinaryHeap};

use crate::level::Level;

#[derive(Debug)]
pub struct Asks(BinaryHeap<Reverse<Level>>);

impl Asks {
    pub fn new() -> Self {
        Self(BinaryHeap::new())
    }
}
