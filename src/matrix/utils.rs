use crate::matrix::Matrix;
use crate::vector::{Mappers, zipeach};

pub trait Utils<R>: IntoIterator<Item=R> where
    R: IntoIterator
{
    fn size(self) -> (usize, usize) where
        Self: Sized,
        Self::IntoIter: ExactSizeIterator<Item=R>,
        R::IntoIter: ExactSizeIterator<Item=R::Item>
    {
        let rows = &mut self.into_iter();
        let height = rows.len();
        let width = rows.next().map_or(0, |row| row.into_iter().len());
        (height, width)
    }

    fn lazy_size(self) -> (usize, usize) where
        Self: Sized,
    {
        let rows = &mut self.into_iter();
        rows.next()
            .map_or_else(
                || (0, 0),
                |row| (rows.count() + 1, row.into_iter().count()),
            )
    }

    fn transpose(self) -> Matrix<R::Item> where
        Self: Sized,
        Self::IntoIter: Iterator<Item=R>,
        R::IntoIter: Iterator<Item=R::Item>
    {
        let rows = &mut self.into_iter();
        rows.next().map_or_else(
            || vec![],
            |init| {
                let mut cols = init.mapper(|x| vec![x]);
                for row in rows {
                    zipeach(&mut cols, row, |col, x| { col.push(x) })
                }
                cols
            })
    }
}

impl<R, M> Utils<R> for M where
    M: IntoIterator<Item=R>,
    R: IntoIterator,
{}

pub fn size<M, R>(mx: M) -> (usize, usize) where
    M: IntoIterator<Item=R>,
    R: IntoIterator,
    M::IntoIter: ExactSizeIterator<Item=R>,
    R::IntoIter: ExactSizeIterator<Item=R::Item>
{ mx.size() }

pub fn transpose<M, R>(mx: M) -> Matrix<R::Item> where
    M: IntoIterator<Item=R>,
    R: IntoIterator,
    M::IntoIter: Iterator<Item=R>,
    R::IntoIter: Iterator<Item=R::Item>
{ mx.transpose() }

#[cfg(test)]
mod transpose_tests {
    use std::collections::HashMap;

    use crate::entries::IntoHashmap;

    use super::*;

    fn create_candidates() -> HashMap<String, Matrix<i32>> {
        vec![
            ("matrix 3x3".to_owned(), vec![
                vec![1, 1, 1],
                vec![2, 2, 2],
                vec![3, 3, 3]
            ]),
            ("column 3x1".to_owned(), vec![
                vec![1],
                vec![2],
                vec![3]
            ]),
            ("row 1x3".to_owned(), vec![
                vec![1, 1, 1],
            ]),
            ("empty 1x0".to_owned(), vec![vec![]]),
            ("empty 0x0".to_owned(), vec![]),
        ].into_hashmap()
    }

    #[cfg(test)]
    mod transpose_tests {
        use super::*;

        #[test]
        fn transpose_2d_vector() {
            let candidates = create_candidates();
            for (name, mx) in &candidates {
                let result = transpose(mx);
                println!("{}: {:?} from {:?}", name, result, mx);
            }
        }

        #[test]
        fn transpose_2d_array() {
            let candidates = vec![
                [
                    [1, 1, 1],
                    [2, 2, 2],
                    [3, 3, 3]
                ],
            ];
            for mx in candidates {
                let result = transpose(&mx);
                println!("{:?}", result);
            }
        }
    }

    #[cfg(test)]
    mod size_tests {
        use super::*;

        #[test]
        fn size_of_2d_vector() {
            let candidates = create_candidates();
            for (name, mx) in candidates {
                let result = mx.size();
                println!("{} {:?}", name, result);
            }
        }

        #[test]
        fn lazy_size_of_2d_vector() {
            let candidates = create_candidates();
            for (name, mx) in &candidates {
                let result = mx.lazy_size();
                println!("{} {:?}", name, result);
            }
        }
    }
}

