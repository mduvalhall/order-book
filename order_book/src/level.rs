use std::{cmp::Ordering, collections::VecDeque};

use crate::order::Order;

#[derive(Debug)]
pub struct Level {
    price: u32,
    orders: VecDeque<Order>,
}

impl Level {
    pub fn new(price: u32) -> Self {
        Self {
            price,
            orders: VecDeque::new(),
        }
    }

    pub fn pop(&mut self) -> Option<Order> {
        self.orders.pop_front()
    }

    pub fn add(&mut self, order: Order) {
        todo!()
    }
}

impl PartialEq for Level {
    fn eq(&self, other: &Self) -> bool {
        self.price.cmp(&other.price) == Ordering::Equal
    }
}

impl Eq for Level {}

impl PartialOrd for Level {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.price.cmp(&other.price))
    }
}

impl Ord for Level {
    fn cmp(&self, other: &Self) -> Ordering {
        self.price.cmp(&other.price)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Reverse;

    #[test]
    fn ordering_is_correct() {
        let level_a = Level::new(10);
        let level_b = Level::new(20);
        assert!((level_a < level_b) == true);
    }

    #[test]
    fn reverse_ordering_is_correct() {
        let level_a = Reverse(Level::new(10));
        let level_b = Reverse(Level::new(20));
        assert!((level_b < level_a) == true);
    }
}
