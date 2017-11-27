use assassin::money::Money;
use assassin::order::Order;
use assassin::traits::*;

pub struct NullCommission {}

impl NullCommission {
    pub fn new() -> NullCommission {
        NullCommission{}
    }
}

impl Commission for NullCommission {
    fn commission_for(&self, _order: &Order) -> Money {
        Money::zero()
    }
}