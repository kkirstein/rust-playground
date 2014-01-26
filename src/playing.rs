// playing.rs
// playground for Rust programming
//
// vim: ft=rust sw=4 ts=4

//#[feature(globs)]

// import timing functions
extern mod extra;

use extra::time::precise_time_ns;
use fibonacci::{fib_rec, fib, fib_bigint};
use perfect_number::perfect_numbers;

// define modules
mod fibonacci;
mod perfect_number;
mod mandelbrot;


// Main
// ====
fn main() {
    println("Fibonacci series:");
	println("=================");

	let tic = precise_time_ns();
	println!("fib(35) = {:u} (naive recursive version)", fib_rec(35));
	let toc = precise_time_ns();
	println!("Elapsed time {:u}ms", (toc-tic)/1000000);
	println("");

	let tic = precise_time_ns();
	println!("fib(35) = {:f}", fib(35));
	let toc = precise_time_ns();
	println!("Elapsed time {:u}us", (toc-tic)/1000);
	println("");

	let tic = precise_time_ns();
	println!("fib(1000) = {:f}", fib(1000));
	let toc = precise_time_ns();
	println!("Elapsed time {:u}us", (toc-tic)/1000);
	println("");

	let tic = precise_time_ns();
	println!("fib_bigint(1000) = {:s}", fib_bigint(1000).to_str());
	let toc = precise_time_ns();
	println!("Elapsed time {:u}ms", (toc-tic)/1000000);
	println("");

    println("Perfect numbers:");
	println("================");

	let tic = precise_time_ns();
	let pn = perfect_numbers(10000);
	println!("Found {:u} perfect numbers below 10000:", pn.len());
	println!("{:?}", pn);
	let toc = precise_time_ns();
	println!("Elapsed time {:u}ms", (toc-tic)/1000000);
	println("");
}


