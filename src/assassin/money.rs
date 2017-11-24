use std::ops::{Add, Sub, Mul, Div};

struct Money {
	cents: i32,
}

impl Money {
	pub fn new(cents: i32) -> Money {
		Money {
			cents: cents,
		}
	}

	pub fn dollars(&self) -> i32 {
		self.cents / 100
	}

	pub fn cents(&self) -> i32 {
		self.cents % 100
	}
}

impl Add for Money {
	type Output = Money;

	fn add(self, rhs: Money) -> Money {
		Money{
			cents: self.cents + rhs.cents,
		}
	}
}

impl Sub for Money {
	type Output = Money;

	fn sub(self, rhs: Money) -> Money {
		Money{
			cents: self.cents - rhs.cents,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_dollars() {
		assert!(Money::new(1223).dollars() == 12);
	}

	#[test]
	fn test_cents() {
		assert!(Money::new(1223).cents() == 23);
	}

	#[test]
	fn test_add() {
		let m1 = Money::new(99);
		let m2 = Money::new(115);

		let m3 = m1 + m2;

		assert!(m3.dollars() == 2);
		assert!(m3.cents() == 14);
	}

	#[test]
	fn test_sub() {
		let m1 = Money::new(115);
		let m2 = Money::new(20);

		let m3 = m1 - m2;

		assert!(m3.dollars() == 0);
		assert!(m3.cents() == 95);
	}

	// #[test]
	// fn add_should_work() {

	// 	// normal
	// 	let mut p1 = Price::new(1, 0);
	// 	let p2 = Price::new(1, 10);

	// 	p1.add(&p2);

	// 	assert!(p1.dollars == 2);
	// 	assert!(p1.cents == 10);

	// 	// carried value
	// 	let mut p3 = Price::new(1, 99);
	// 	let p4 = Price::new(0, 1);

	// 	p3.add(&p4);

	// 	assert!(p3.dollars == 2);
	// 	assert!(p3.cents == 0);
	// }

	// fn subtract_should_work() {

	// 	// normal
	// 	let mut p5 = Price::new(1, 99);
	// 	let p6 = Price::new(0, 45);

	// 	p5.subtract(&p6);

	// 	assert!(p5.dollars == 0);
	// 	assert!(p5.cents == 44);

	// 	// carried value
	// 	let mut p5 = Price::new(1, 15);
	// 	let p6 = Price::new(1, 16);

	// 	p5.subtract(&p6);

	// 	assert!(p5.dollars == -1);
	// 	assert!(p5.cents == 99);
	// }
}
