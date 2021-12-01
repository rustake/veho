use crate::matrix::Matrix;

pub fn init<T, F: Fn(usize, usize) -> T>(height: usize, width: usize, f: F) -> Matrix<T> {
    (0..height).map(
        |i| (0..width).map(
            |j| f(i, j)
        ).collect()
    ).collect()
}

pub fn iso<T: Clone>(height: usize, width: usize, value: T) -> Matrix<T> {
    vec![vec![value; width]; height]
}

#[cfg(test)]
mod create_matrix_tests {
    use crate::matrix::inits::{init, iso};

    #[test]
    fn test_init() {
        let sample_matrix = init(4, 3, |i, j| (i + j) as i32);
        println!(">> {:?}", sample_matrix);
    }

    #[test]
    fn test_iso() {
        let sample_matrix = iso(4, 3, 0);
        println!(">> {:?}", sample_matrix);
    }
}