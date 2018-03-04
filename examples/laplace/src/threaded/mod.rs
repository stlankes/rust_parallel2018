use rayon::prelude::*;
use rayon;

pub mod compute_unsafe;
pub use self::compute_unsafe::compute_unsafe;

pub mod compute;
pub use self::compute::compute;

fn compute_chunk_size(size_x: usize, size_y: usize) -> usize {
	let number_of_threads = rayon::current_num_threads();
	let chunk_size = if size_y % number_of_threads == 0 {
		(size_y / number_of_threads) * size_x
	} else {
		((size_y + (number_of_threads - (size_y % number_of_threads))) / number_of_threads) * size_x
	};

	chunk_size
}

fn get_residual(matrix: &[f64], size_x: usize, size_y: usize) -> f64 {
	let sum = (1..size_y-1).into_par_iter()
		.map(|y| {
			let mut local_sum = 0.0;

			for x in 1..(size_x - 1) {
				let new = (matrix[y * size_x + x - 1] + matrix[y * size_x + x + 1]
					+ matrix[(y + 1) * size_x + x]
					+ matrix[(y - 1) * size_x + x]) * 0.25;

				let diff = new - matrix[y * size_x + x];
				local_sum += diff * diff;
			}

			local_sum
		}).sum();

	sum
}
