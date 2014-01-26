// fibonacci.rs
// Implementation of fib module
// to calculate Fibbonacci series
//
// vim: ft=rust sw=4 ts=4

use extra::bigint::BigUint;
use std::num::{Zero,One};

// Fibonacci series
// ================

// Naive implementation
pub fn fib_rec(n: uint) -> uint {
	match n {
		0 | 1  	=> n,
		_		=> fib_rec(n-2) + fib_rec(n-1)
	}
}

// Linear recursive implementation
pub fn fib(n: uint) -> f64 {
	fn fib_aux(a: f64, b: f64, n: uint) -> f64 {
		match n {
			0 => a,
			_ => fib_aux(b, (a+b), n-1)
		}
	}

	fib_aux(0.0, 1.0, n)
}
pub fn fib_bigint(n: uint) -> BigUint {
	fn fib_aux(a: BigUint, b: BigUint, n: uint) -> BigUint {
		match n {
			0 => a,
			_ => fib_aux(b.clone(), (a+b), n-1)
		}
	}

	fib_aux(Zero::zero(), One::one(), n)
}

