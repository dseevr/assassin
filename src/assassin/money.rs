use std::fmt;
use std::ops::{Add, Sub, Mul, Div};
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};
use std::iter::{Iterator, Sum};
use std::cmp::{PartialEq, PartialOrd, Ordering};

use assassin::util::add_commas;

#[derive(Copy,Clone)]
pub struct Money {
	cents: i32,
}

impl Money {
	pub fn new(dollars: i32, cents: i32) -> Money {
		if cents < 0 || cents > 99 {
			panic!("cents must be >= 0 and <= 99")
		}

		let value = if dollars < 0 {
			dollars * 100 - cents
		} else {
			dollars * 100 + cents
		};

		Money::from_cents(value)
	}

	pub fn from_cents(cents: i32) -> Money {
		Money{
			cents: cents,
		}
	}

	pub fn from_float(f: f32) -> Money {
		Money{
			cents: (f * 100.0).round() as i32
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

impl Mul<i32> for Money {
	type Output = Money;

	fn mul(self, rhs: i32) -> Money {
		Money{
			cents: self.cents * rhs,
		}
	}
}

impl Div<i32> for Money {
	type Output = Money;

	fn div(self, rhs: i32) -> Money {
		let cents = (self.cents as f32 / rhs as f32).round() as i32;

		Money{
			cents: cents,
		}
	}
}

impl AddAssign for Money {
	fn add_assign(&mut self, rhs: Money) {
		self.cents = self.cents + rhs.cents;
	}
}

impl SubAssign for Money {
	fn sub_assign(&mut self, rhs: Money) {
		self.cents = self.cents - rhs.cents;
	}
}

impl MulAssign<i32> for Money {
	fn mul_assign(&mut self, rhs: i32) {
		self.cents *= rhs;
	}
}

impl DivAssign<i32> for Money {
	fn div_assign(&mut self, rhs: i32) {
		// round up to nearest cent when dividing
		self.cents = (self.cents as f32 / rhs as f32).round() as i32;
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

impl Sum for Money {
	fn sum<I>(iter: I) -> Money where I: Iterator<Item=Money> {
		iter.fold(Money::zero(), Add::add)
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
	fn test_constructors() {
		let m1 = Money::new(1, 23);
		let m2 = Money::from_cents(1_23);

		assert!(m1 == m2);
	}

	#[test]
	fn test_dollars() {
		assert!(Money::new(12, 23).dollars() == 12);
	}

	#[test]
	fn test_cents() {
		assert!(Money::new(12, 23).cents() == 23);
	}

	#[test]
	fn test_add() {
		let m1 = Money::new(0, 99);
		let m2 = Money::new(1, 15);

		let m3 = m1 + m2;

		assert!(m3.dollars() == 2);
		assert!(m3.cents() == 14);
	}

	#[test]
	fn test_sub() {
		let m1 = Money::new(1, 15);
		let m2 = Money::new(0, 20);

		let m3 = m1 - m2;

		assert!(m3.dollars() == 0);
		assert!(m3.cents() == 95);
	}

	#[test]
	fn test_mul() {
		let mut m = Money::new(1, 10);
		m = m * 5;

		assert!(m.dollars() == 5);
		assert!(m.cents() == 50);
	}

	#[test]
	fn test_div() {
		let mut m = Money::new(1, 99);
		m = m / 5;

		assert!(m.dollars() == 0);
		assert!(m.cents() == 40);
	}

	#[test]
	fn test_add_assign() {
		let mut m = Money::new(1, 10);
		m += Money::new(0, 10);

		assert!(m.dollars() == 1);
		assert!(m.cents() == 20);
	}

	#[test]
	fn test_sub_assign() {
		let mut m = Money::new(1, 10);
		m -= Money::new(0, 5);

		assert!(m.dollars() == 1);
		assert!(m.cents() == 5);
	}

	#[test]
	fn test_mul_assign() {
		let mut m = Money::new(1, 10);
		m *= 5;

		assert!(m.dollars() == 5);
		assert!(m.cents() == 50);
	}

	#[test]
	fn test_div_assign() {
		let mut m = Money::new(1, 99);
		m /= 5;

		assert!(m.dollars() == 0);
		assert!(m.cents() == 40);
	}

	#[test]
	fn test_sum() {
		let m1 = Money::new(0, 10);
		let m2 = Money::new(0, 5);

		let ms = vec![m1, m2];
		let sum: Money = ms.iter().cloned().sum();

		assert!(sum == m1 + m2);
	}

	#[test]
	fn test_equality() {
		let m1 = Money::new(1, 15);
		let m2 = Money::new(1, 15);
		let m3 = Money::new(1, 16);

		assert!(m1 == m2);
		assert!(m1 != m3);
	}

	#[test]
	fn test_ordering() {
		let large = Money::new(10, 50);
		let same  = Money::new(10, 50);
		let small = Money::new(0, 25);

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
			let res = format!("{}", Money::from_cents(cents));

			println!("Expected: {}", s);
			println!("Got: {}", res);

			assert!(res == s);
		}

		println!("");

		test(0,          "$0.00");
		test(1,          "$0.01");
		test(11,         "$0.11");
		test(111,        "$1.11");
		test(1111,       "$11.11");
		test(11111,      "$111.11");
		test(111111,     "$1,111.11");
		test(1111111,    "$11,111.11");
		test(11111111,   "$111,111.11");
		test(111111111,  "$1,111,111.11");
		test(1111111111, "$11,111,111.11");

		test(-0,          "$0.00");
		test(-1,          "-$0.01");
		test(-11,         "-$0.11");
		test(-111,        "-$1.11");
		test(-1111,       "-$11.11");
		test(-11111,      "-$111.11");
		test(-111111,     "-$1,111.11");
		test(-1111111,    "-$11,111.11");
		test(-11111111,   "-$111,111.11");
		test(-111111111,  "-$1,111,111.11");
		test(-1111111111, "-$11,111,111.11");
	}
}
