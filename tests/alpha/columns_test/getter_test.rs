// use std::ops::Index;
//
// use veho::matrix::init;
//
//
// pub fn column<R, M>(matrix: M, c: usize) -> Vec<R::Item> where
//     M: IntoIterator<Item=R>,
//     R: IntoIterator + Index<usize, Output=R::Item>,
// // R::IntoIter: Iterator<Item=R::Item>
// { matrix.into_iter().map(|row| row[c]).collect() }
//
// #[test]
// fn test_column_getter() {
//     let matrix = init(4, 4, |i, j| i + j);
//     println!("{:?}", column(matrix, 2));
// }