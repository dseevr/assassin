use assassin::money::Money;
use assassin::filled_order::FilledOrder;
use assassin::traits::*;

pub struct NullCommission {}

impl NullCommission {
    pub fn new() -> NullCommission {
        NullCommission {}
    }
}

impl Commission for NullCommission {
    fn commission_for(&self, _filled_order: &FilledOrder) -> Money {
        Money::zero()
    }
}
