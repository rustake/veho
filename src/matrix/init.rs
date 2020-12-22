use crate::matrix::alias::Matrix;

pub fn init<T, F: Fn(usize, usize) -> T>(height: usize, width: usize, f: F) -> Matrix<T> {
    return (0..height).map(
        |i| (0..width).map(
            |j| f(i, j)
        ).collect()
    ).collect();
}