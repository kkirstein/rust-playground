// perfect_number.rs
// Implementation of perfect_numbers module
// to calculate perfect numbers
//
// vim: ft=rust sw=4 ts=4

use std::iter::range_inclusive;

// Perfect numbers
// ===============
fn is_perfect(&n: &uint) -> bool {
	let mut i = 1;
	let mut sum = 0;

	while (i < n) {
		if (n % i) == 0 {
			sum += i;
		}
		i += 1;
	}

	sum == n
}

pub fn perfect_numbers(n: uint) -> ~[uint] {
	range_inclusive(0,n).filter(is_perfect).to_owned_vec()
}


