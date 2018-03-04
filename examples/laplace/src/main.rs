// A Laplace solver using simple Jacobi iteration

extern crate time;
extern crate rayon;

use time::PreciseTime;

use std::vec;

mod sequential;
mod ray;
mod threaded;
mod threadpool;

fn main() {
	let size_x = 2500;
	let size_y = 2500;

	run(size_x, size_y, "sequential", sequential::compute);
	run(size_x, size_y, "sequential unsafe", sequential::compute_unsafe);

	println!("Number of rayon threads: {}", rayon::current_num_threads());
	run(size_x, size_y, "rayon", ray::compute);
	run(size_x, size_y, "rayon unsafe", ray::compute_unsafe);

	run(size_x, size_y, "threaded", threaded::compute);
	run(size_x, size_y, "threaded unsafe", threaded::compute_unsafe);

	run(size_x, size_y, "threadpool", threadpool::compute);
	run(size_x, size_y, "threadpool", threadpool::compute_unsafe);
}

fn run(
    size_x: usize,
    size_y: usize,
    name: &'static str,
    compute: fn(vec::Vec<vec::Vec<f64>>, usize, usize) -> (usize, f64)
) {
    let matrix = matrix_setup(size_x, size_y);

    let start = PreciseTime::now();
    let (iterations, res) = compute(matrix, size_x, size_y);
    let end = PreciseTime::now();

    println!(
        "{} seconds for {} version ({} iterations, {} res)",
        start.to(end),
        name,
        iterations,
        res
    );
}

fn matrix_setup(size_x: usize, size_y: usize) -> (vec::Vec<vec::Vec<f64>>) {
    let mut matrix = vec![vec![0.0; size_x * size_y]; 2];

    // top row
    for x in 0..size_x {
        matrix[0][x] = 1.0;
        matrix[1][x] = 1.0;
    }

    // bottom row
    for x in 0..size_x {
        matrix[0][(size_y - 1) * size_x + x] = 1.0;
        matrix[1][(size_y - 1) * size_x + x] = 1.0;
    }

    // left row
    for y in 0..size_y {
        matrix[0][y * size_x] = 1.0;
        matrix[1][y * size_x] = 1.0;
    }

    // right row
    for y in 0..size_y {
        matrix[0][y * size_x + size_x - 1] = 1.0;
        matrix[1][y * size_x + size_x - 1] = 1.0;
    }

    matrix
}
