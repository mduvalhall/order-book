use crate::order::{Id, Order, Side};

#[derive(Debug)]
pub struct Trade<S: Side> {
    id: Id,
    orders: Vec<Order<S::Opposite>>,
}

impl<S: Side> Trade<S> {
    pub fn new(id: Id) -> Self {
        Self {
            id,
            orders: Vec::new(),
        }
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn orders(&self) -> &Vec<Order<S::Opposite>> {
        &self.orders
    }

    pub fn add_order(&mut self, order: Order<S::Opposite>) {
        self.orders.push(order);
    }
}

impl<S: Side> From<&Order<S>> for Trade<S::Opposite> {
    fn from(order: &Order<S>) -> Self {
        Self {
            id: order.id(),
            orders: Vec::new(),
        }
    }
}
