pub mod compute_unsafe;
pub use self::compute_unsafe::compute_unsafe;

pub mod compute;
pub use self::compute::compute;

fn get_residual(matrix: &[f64], size_x: usize, size_y: usize) -> f64 {
    let mut sum = 0.0;

    for y in 1..(size_y - 1) {
        for x in 1..(size_x - 1) {
            let new = (matrix[y * size_x + x - 1] + matrix[y * size_x + x + 1]
                + matrix[(y + 1) * size_x + x]
                + matrix[(y - 1) * size_x + x]) * 0.25;

            let diff = new - matrix[y * size_x + x];
            sum += diff * diff;
        }
    }

    sum
}
