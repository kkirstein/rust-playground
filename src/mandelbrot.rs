// mandelbrot.rs
// Implementation of mandelbrot module
// to calculate Mandelbrot sets
//
// vim: ft=rust sw=4, ts=4

use extra::complex::Cmplx;

// define colormap as static veriable
pub static colormap: &'static[(u8, u8, u8)] = &[
	(0, 0, 0), (0, 0, 252), (64, 0, 252), (124, 0, 252), (188, 0, 252),
	(252, 0, 252), (252, 0, 188), (252, 0, 124), (252, 0, 64), (252, 0, 0),
	(252, 64, 0), (252, 124, 0), (252, 188, 0), (252, 252, 0), (188, 252, 0),
	(124, 252, 0), (64, 252, 0), (0, 252, 0), (0, 252, 64), (0, 252, 124),
	(0, 252, 188), (0, 252, 252), (0, 188, 252), (0, 124, 252), (0, 64, 252),
	(124, 124, 252), (156, 124, 252), (188, 124, 252), (220, 124, 252), (252, 124, 252),
	(252, 124, 220), (252, 124, 188), (252, 124, 156), (252, 124, 124), (252, 156, 124),
	(252, 188, 124), (252, 220, 124), (252, 252, 124), (220, 252, 124), (188, 252, 124),
	(156, 252, 124), (124, 252, 124), (124, 252, 156), (124, 252, 188), (124, 252, 220),
	(124, 252, 252), (124, 220, 252), (124, 188, 252), (124, 156, 252), (180, 180, 252),
	(196, 180, 252), (216, 180, 252), (232, 180, 252), (252, 180, 252), (252, 180, 232),
	(252, 180, 216), (252, 180, 196), (252, 180, 180), (252, 196, 180), (252, 216, 180),
	(252, 232, 180), (252, 252, 180), (232, 252, 180), (216, 252, 180), (196, 252, 180),
	(180, 252, 180), (180, 252, 196), (180, 252, 216), (180, 252, 232), (180, 252, 252),
	(180, 232, 252), (180, 216, 252), (180, 196, 252), (0, 0, 112), (28, 0, 112),
	(56, 0, 112), (84, 0, 112), (112, 0, 112), (112, 0, 84), (112, 0, 56),
	(112, 0, 28), (112, 0, 0), (112, 28, 0), (112, 56, 0), (112, 84, 0),
	(112, 112, 0), (84, 112, 0), (56, 112, 0), (28, 112, 0), (0, 112, 0),
	(0, 112, 28), (0, 112, 56), (0, 112, 84), (0, 112, 112), (0, 84, 112),
	(0, 56, 112), (0, 28, 112), (56, 56, 112), (68, 56, 112), (84, 56, 112),
	(96, 56, 112), (112, 56, 112), (112, 56, 96), (112, 56, 84), (112, 56, 68),
	(112, 56, 56), (112, 68, 56), (112, 84, 56), (112, 96, 56), (112, 112, 56),
	(96, 112, 56), (84, 112, 56), (68, 112, 56), (56, 112, 56), (56, 112, 68),
	(56, 112, 84), (56, 112, 96), (56, 112, 112), (56, 96, 112), (56, 84, 112),
	(56, 68, 112), (80, 80, 112), (88, 80, 112), (96, 80, 112), (104, 80, 112),
	(112, 80, 112), (112, 80, 104), (112, 80, 96), (112, 80, 88), (112, 80, 80),
	(112, 88, 80), (112, 96, 80), (112, 104, 80), (112, 112, 80), (104, 112, 80),
	(96, 112, 80), (88, 112, 80), (80, 112, 80), (80, 112, 88), (80, 112, 96),
	(80, 112, 104), (80, 112, 112), (80, 104, 112), (80, 96, 112), (80, 88, 112),
	(0, 0, 64), (16, 0, 64), (32, 0, 64), (48, 0, 64), (64, 0, 64), (64, 0, 48),
	(64, 0, 32), (64, 0, 16), (64, 0, 0), (64, 16, 0), (64, 32, 0), (64, 48, 0),
	(64, 64, 0), (48, 64, 0), (32, 64, 0), (16, 64, 0), (0, 64, 0), (0, 64, 16),
	(0, 64, 32), (0, 64, 48), (0, 64, 64), (0, 48, 64), (0, 32, 64), (0, 16, 64),
	(32, 32, 64), (40, 32, 64), (48, 32, 64), (56, 32, 64), (64, 32, 64),
	(64, 32, 56), (64, 32, 48), (64, 32, 40), (64, 32, 32), (64, 40, 32),
	(64, 48, 32), (64, 56, 32), (64, 64, 32), (56, 64, 32), (48, 64, 32),
	(40, 64, 32), (32, 64, 32), (32, 64, 40), (32, 64, 48), (32, 64, 56),
	(32, 64, 64), (32, 56, 64), (32, 48, 64), (32, 40, 64), (44, 44, 64),
	(48, 44, 64), (52, 44, 64), (60, 44, 64), (64, 44, 64), (64, 44, 60),
	(64, 44, 52), (64, 44, 48), (64, 44, 44), (64, 48, 44), (64, 52, 44),
	(64, 60, 44), (64, 64, 44), (60, 64, 44), (52, 64, 44), (48, 64, 44),
	(44, 64, 44), (44, 64, 48), (44, 64, 52), (44, 64, 60), (44, 64, 64),
	(44, 60, 64), (44, 52, 64), (44, 48, 64)];
//static n_max: uint = colormap.len();
static n_max:uint = 217u;
static r_max: f32 = 2.0;

// calculate pixel value
fn pixel_value(x: f32, y: f32) -> uint {

	let z0 = Cmplx::new(x, y);
	//let mut z: Cmplx<f32> = Cmplx::zero();
	let mut z: Cmplx<f32>  = Cmplx::new(0f32, 0f32);
	let mut n: uint = 0;

	loop {
		z = z.mul(&z).add(&z0);
		if ((z.norm() > r_max) || (n > n_max)) {
			break;
		}
		n += 1;
	}

	n
}

// generate Mandelbrot set
pub fn mandelbrot(x_max: uint, y_max: uint, x_center: f32, y_center:f32, pixel_size: f32) -> ~[uint] {
	use std::vec;

	let mut mset = vec::with_capacity(x_max * y_max);
	let x_offset:f32 = x_center + 0.5*pixel_size*(x_max.to_f32().unwrap()+1f32);
	let y_offset:f32 = y_center - 0.5*pixel_size*(y_max.to_f32().unwrap()+1f32);

	for y in range(0, y_max) {
		for x in range(0, x_max) {
			mset.push(pixel_value(x_offset + x.to_f32().unwrap()*pixel_size, y_offset - y.to_f32().unwrap()*pixel_size));
		}
	}

	mset
}

