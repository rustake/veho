use std::fmt;

use veho::matrix::{init, Matrix};
use veho::vector::Mappers;

pub fn transpose<R, M>(matrix: M) -> Matrix<R::Item> where
    M: IntoIterator<Item=R>,
    R: IntoIterator,
    R::IntoIter: ExactSizeIterator<Item=R::Item>,
{
    let mut iters = (&mut matrix.into_iter())
        .map(|row| row.into_iter())
        .collect::<Vec<R::IntoIter>>();
    let width = iters.first().map_or(0, |iter| iter.len());
    (0..width).map(|_| {
        (&mut iters).mapper(|it| it.next().unwrap())
    }).collect::<Matrix<R::Item>>()
}

pub fn columns_mapper<R, M, T, F>(matrix: M, mut f: F) -> Matrix<T> where
    M: IntoIterator<Item=R>,
    R: IntoIterator,
    R::IntoIter: ExactSizeIterator<Item=R::Item>,
    F: FnMut(R::Item) -> T,
{
    let mut iters = (&mut matrix.into_iter())
        .map(|row| row.into_iter())
        .collect::<Vec<R::IntoIter>>();
    let width = iters.first().map_or(0, |iter| iter.len());
    (0..width).map(|_|
        (&mut iters).mapper(|it| f(it.next().unwrap()))
    ).collect::<Matrix<T>>()
}

#[test]
fn test_columns_mapper() {
    let matrix = init(4, 4, |i, j| (i + 1) * 10 + j * 2);
    println!("columns_mapped {:?}", columns_mapper(&matrix, |x| x * 2));
    println!("transposed {:?}", transpose(&matrix));
    println!("original {:?}", matrix)
    // println!("{:?}", column(matrix, 2));
}