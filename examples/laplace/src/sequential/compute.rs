use std::vec;
use sequential::get_residual;

#[inline(never)]
pub fn compute(mut matrix: vec::Vec<vec::Vec<f64>>, size_x: usize, size_y: usize) -> (usize, f64) {
    let mut counter = 0;
    while counter < 1000 {
        {
            // allow a borrow and a reference to the same vector
            let (current, next) = matrix.split_at_mut(1);

            iteration(&current[0], &mut next[0], size_x, size_y);
        }
        matrix.swap(0, 1);

        counter += 1;
    }

    (counter, get_residual(&matrix[0], size_x, size_y))
}

fn iteration(cur: &[f64], next: &mut [f64], size_x: usize, size_y: usize) {
    for y in 1..size_y - 1 {
        let offset_base = y * size_x;
        for x in 1..size_x - 1 {
            next[offset_base + x] = (cur[offset_base + x - 1] + cur[offset_base + x + 1]
                + cur[offset_base + size_x + x]
                + cur[offset_base - size_x + x]) * 0.25;
        }
    }
}
