// playing.rs
// playground for Rust programming
//
// vim: ft=rust sw=4 ts=4

//#[feature(globs)]

// import timing functions
extern mod extra;

use extra::bigint::BigUint;
use std::num::{Zero,One};
use extra::time::precise_time_ns;

// Fibonacci series

// Naive implementation
fn fib_rec(n: uint) -> uint {
	match n {
		0 | 1  	=> n,
		_		=> fib_rec(n-2) + fib_rec(n-1)
	}
}

// Linear recursive implementation
fn fib(n: uint) -> f64 {
	fn fib_aux(a: f64, b: f64, n: uint) -> f64 {
		match n {
			0 => a,
			_ => fib_aux(b, (a+b), n-1)
		}
	}

	fib_aux(0.0, 1.0, n)
}
fn fib_bigint(n: uint) -> BigUint {
	fn fib_aux(a: BigUint, b: BigUint, n: uint) -> BigUint {
		match n {
			0 => a,
			_ => fib_aux(b.clone(), (a+b), n-1)
		}
	}

	fib_aux(Zero::zero(), One::one(), n)
}


fn main() {
    println("Fibonacci series:");
	println("=================");

	let tic = precise_time_ns();
	println!("fib(35) = {:u}", fib_rec(35));
	let toc = precise_time_ns();
	println!("Elapsed time {:u}ms", (toc-tic)/1000000);
	println("");

	let tic = precise_time_ns();
	println!("fib(35) = {:f}", fib(35));
	let toc = precise_time_ns();
	println!("Elapsed time {:u}ms", (toc-tic)/1000000);
	println("");

	let tic = precise_time_ns();
	println!("fib(1000) = {:f}", fib(1000));
	let toc = precise_time_ns();
	println!("Elapsed time {:u}ms", (toc-tic)/1000000);
	println("");

	let tic = precise_time_ns();
	println!("fib_bigint(1000) = {:s}", fib_bigint(1000).to_str());
	let toc = precise_time_ns();
	println!("Elapsed time {:u}ms", (toc-tic)/1000000);
	println("");
}


