use std::marker::PhantomData;

#[derive(Debug)]
pub struct Ask;

impl Side for Ask {
    type Opposite = Bid;
}

impl sealed::Sealed for Ask {}

#[derive(Debug)]
pub struct Bid;

impl Side for Bid {
    type Opposite = Ask;
}

impl sealed::Sealed for Bid {}

#[derive(Debug)]
pub struct Order<T: Side> {
    id: Id,
    price: Price,
    quantity: Quantity,
    _state: PhantomData<T>,
}

mod sealed {
    pub trait Sealed {}
}

pub trait Side: sealed::Sealed {
    type Opposite: Side;
}

impl<T: Side> Order<T> {
    pub fn new(id: Id, price: Price, quantity: Quantity) -> Self {
        Self {
            id,
            price,
            quantity,
            _state: PhantomData,
        }
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn price(&self) -> Price {
        self.price
    }

    pub fn quantity(&self) -> Quantity {
        self.quantity
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Id(u64);

impl Id {
    pub fn new(_0: u64) -> Self {
        Self(_0)
    }

    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Price(u64);

impl Price {
    pub fn new(_0: u64) -> Self {
        Self(_0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Quantity(u64);

impl Quantity {
    pub fn new(_0: u64) -> Self {
        Self(_0)
    }
}
