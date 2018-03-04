extern crate crossbeam;

use rayon;
use std::vec;

use threaded::get_residual;
use threaded::compute_chunk_size;

#[inline(never)]
pub fn compute_unsafe(mut matrix: vec::Vec<vec::Vec<f64>>, size_x: usize, size_y: usize) -> (usize, f64) {
    let mut counter = 0;
    let chunk_size = compute_chunk_size(size_x, size_y - 2);

    while counter < 1000 {
        crossbeam::scope(|scope| {
			// use the same number of threads like the rayon implementation
            let mut threads = Vec::with_capacity(rayon::current_num_threads());
            {
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

                    threads.push(scope.spawn(move || {
                        iteration(&current, m_out_split, size_x, global_index);
                    }));

                    global_index += chunk_size;
                }
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
			unsafe {
				*next.get_unchecked_mut(offset_next + x) = (*cur.get_unchecked(global_offset_base + x - 1)
					+ *cur.get_unchecked(global_offset_base + x + 1)
					+ *cur.get_unchecked(global_offset_base + size_x + x)
					+ *cur.get_unchecked(global_offset_base - size_x + x)) * 0.25;
			}
        }
    }
}
