// use std::ops::{Fn};
use crate::matrix::Matrix;

pub fn mapper<T, P, F: Fn(&T) -> P>(matrix: &Matrix<T>, f: F) -> Matrix<P> {
    return matrix.into_iter().map(
        |row| row.into_iter().map(|x| f(x)).collect()
    ).collect();
}

pub fn iterate<T, F: Fn(&T)>(matrix: &Matrix<T>, f: F) {
    for row in matrix.iter() {
        for x in row.iter() {
            f(x);
        }
    }
}

pub fn mutate<T, F: Fn(&T) -> T>(matrix: &mut Matrix<T>, f: F) -> &Matrix<T> {
    for row in matrix.iter_mut() {
        for x in row.iter_mut() {
            *x = f(x);
        }
    }
    return matrix;
}

#[cfg(test)]
mod tests {
    use crate::matrix::mappers::{iterate, mutate, mapper};
    use crate::matrix::alias::Matrix;
    use crate::matrix::inits::init;


    fn create_matrix() -> Matrix<i32> {
        return init(3, 4, |i, j| i as i32 + j as i32);
    }

    #[test]
    fn test_mapper() {
        let matrix = create_matrix();
        println!("original: matrix = {:?}", matrix);
        let matrix = mapper(&matrix, |x| x + 1);
        println!("modified: matrix = {:?}", matrix);
    }

    #[test]
    fn test_iterate() {
        let mut matrix = create_matrix();
        iterate(&mut matrix, |x| println!("{}", x + 1));
    }

    #[test]
    fn test_mutate() {
        let mut matrix = create_matrix();
        println!("original: matrix = {:?}", matrix);
        mutate(&mut matrix, |x| x * 2);
        println!("modified: matrix = {:?}", matrix);
    }
}