#[derive(Debug)]
pub struct Ask(Order);

#[derive(Debug)]
pub struct Bid(Order);

#[derive(Debug)]
pub struct Buy {
    id: Id,
    bids: Vec<Bid>,
}

impl Buy {
    pub fn new(id: Id) -> Self {
        Self {
            id,
            bids: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Sell {
    id: Id,
    asks: Vec<Ask>,
}

impl Sell {
    pub fn new(id: Id) -> Self {
        Self {
            id,
            asks: Vec::new(),
        }
    }
}

#[derive(Debug)]
struct Order {
    id: Id,
    price: Price,
    quantity: Quantity,
}

impl Order {
    fn new(id: Id, price: Price, quantity: Quantity) -> Self {
        Self {
            id,
            price,
            quantity,
        }
    }
}

#[derive(Debug)]
pub struct Id(u64);

impl Id {
    pub fn new(_0: u64) -> Self {
        Self(_0)
    }
}

#[derive(Debug)]
pub struct Price(u64);

impl Price {
    pub fn new(_0: u64) -> Self {
        Self(_0)
    }
}

#[derive(Debug)]
pub struct Quantity(u64);

impl Quantity {
    pub fn new(_0: u64) -> Self {
        Self(_0)
    }
}
