extern crate crossbeam;
extern crate scoped_threadpool;

use rayon;
use std::vec;

use threadpool::get_residual;
use threadpool::compute_chunk_size;

use self::scoped_threadpool::Pool;

#[inline(never)]
pub fn compute(mut matrix: vec::Vec<vec::Vec<f64>>, size_x: usize, size_y: usize) -> (usize, f64) {
    let mut counter = 0;
    let chunk_size = compute_chunk_size(size_x, size_y - 2);

	// use the same number of threads like the rayon implementation
	let mut pool = Pool::new(rayon::current_num_threads() as u32);

    while counter < 1000 {
        pool.scoped(|scope| {
            // get input and output matrix
            let (current, m_out) = matrix.split_at_mut(1);
            let current: &[f64] = &current[0];

            // cut off top and bottom row
            let (_, m_out) = m_out[0].split_at_mut(size_x);
            let end = m_out.len() - size_x;
            let (m_out, _) = m_out.split_at_mut(end);

            // global index of m_out_begin
            let mut global_index = size_x;

            // loop over next chunks
            for chunk in m_out.chunks_mut(chunk_size) {
                let m_out_split = chunk;

                scope.execute(move || {
                    iteration(&current, m_out_split, size_x, global_index);
                });

                global_index += chunk_size;
            }
        });
        counter += 1;
        matrix.swap(0, 1);
    }

    (counter, get_residual(&matrix[0], size_x, size_y))
}

fn iteration(cur: &[f64], next: &mut [f64], size_x: usize, global_index: usize) {
    let y_end = next.len() / size_x;
    for y in 0..y_end {
        let offset_next = y * size_x;
        let global_offset_base = offset_next + global_index;
        for x in 1..size_x - 1 {
            next[offset_next + x] = (cur[global_offset_base + x - 1]
                + cur[global_offset_base + x + 1]
                + cur[global_offset_base + size_x + x]
                + cur[global_offset_base - size_x + x]) * 0.25;
        }
    }
}
