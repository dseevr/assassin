use std::fmt;
use std::ops::{Add, Sub}; //, Mul, Div}; // TODO: <-- implement these
use std::cmp::{PartialEq, PartialOrd, Ordering};

use assassin::util::add_commas;

#[derive(Copy,Clone)]
pub struct Money {
	cents: i32,
}

impl Money {
	pub fn new(cents: i32) -> Money {
		Money {
			cents: cents,
		}
	}

	pub fn from_float(f: f32) -> Money {
		Money{
			cents: (f * 100.0) as i32
		}
	}

	pub fn zero() -> Money {
		Money{
			cents: 0,
		}
	}

	pub fn dollars(&self) -> i32 {
		self.cents / 100
	}

	pub fn cents(&self) -> i32 {
		self.cents % 100
	}

	pub fn raw_value(&self) -> i32 {
		self.cents
	}

	// TODO: implement Mul and Div and get rid of these
	pub fn mul(&mut self, i: i32) {
		self.cents *= i;
	}

	pub fn div(&mut self, i: i32) {
		self.cents /= i;
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

impl PartialEq for Money {
	fn eq(&self, rhs: &Money) -> bool {
		self.cents == rhs.cents
	}

	fn ne(&self, rhs: &Money) -> bool {
		self.cents != rhs.cents
	}
}

impl PartialOrd for Money {
	fn partial_cmp(&self, rhs: &Money) -> Option<Ordering> {
		if self < rhs {
			Some(Ordering::Less)
		} else if self == rhs {
			Some(Ordering::Equal)
		} else {
			Some(Ordering::Greater)
		}
	}

	fn lt(&self, rhs: &Money) -> bool {
		self.cents < rhs.cents
	}

	fn le(&self, rhs: &Money) -> bool {
		self < rhs || self == rhs
	}

	fn gt(&self, rhs: &Money) -> bool {
		self.cents > rhs.cents
	}

	fn ge(&self, rhs: &Money) -> bool {
		self > rhs || self == rhs
	}
}

impl fmt::Display for Money {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let sign = if self.cents < 0 { "-" } else { "" };

		write!(
			f,
			"{}${}.{cents:>0width$}",
			sign,
			add_commas(self.dollars().abs()),
			cents = self.cents().abs(),
			width = 2
		)
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

	#[test]
	fn test_mul() {
		let mut m = Money::new(110);
		m.mul(5);

		assert!(m.dollars() == 5);
		assert!(m.cents() == 50);
	}

	#[test]
	fn test_div() {
		let mut m = Money::new(550);
		m.div(5);

		assert!(m.dollars() == 1);
		assert!(m.cents() == 10);
	}

	#[test]
	fn test_equality() {
		let m1 = Money::new(115);
		let m2 = Money::new(115);
		let m3 = Money::new(116);

		assert!(m1 == m2);
		assert!(m1 != m3);
	}

	#[test]
	fn test_ordering() {
		let large = Money::new(1050);
		let same = Money::new(1050);
		let small = Money::new(25);

		assert_eq!(small.partial_cmp(&large), Some(Ordering::Less));
		assert_eq!(large.partial_cmp(&small), Some(Ordering::Greater));
		assert_eq!(large.partial_cmp(&same), Some(Ordering::Equal));

		assert!(large > small);
		assert!(large >= small);
		assert!(small < large);
		assert!(small <= large);
		assert!(large >= same);
		assert!(large <= same);
	}

	#[test]
	fn test_display() {
		fn test(cents: i32, s: &str) {
			let res = format!("{}", Money::new(cents));

			println!("Expected: {}", s);
			println!("Got: {}", res);

			assert!(res == s);
		}

		println!("");

		test(0, "$0.00");
		test(1, "$0.01");
		test(11, "$0.11");
		test(111, "$1.11");
		test(1111, "$11.11");
		test(11111, "$111.11");
		test(111111, "$1,111.11");
		test(1111111, "$11,111.11");
		test(11111111, "$111,111.11");
		test(111111111, "$1,111,111.11");

		test(-0, "$0.00");
		test(-1, "-$0.01");
		test(-11, "-$0.11");
		test(-111, "-$1.11");
		test(-1111, "-$11.11");
		test(-11111, "-$111.11");
		test(-111111, "-$1,111.11");
		test(-1111111, "-$11,111.11");
		test(-11111111, "-$111,111.11");
		test(-111111111, "-$1,111,111.11");
	}
}

