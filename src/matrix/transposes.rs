use crate::matrix::Matrix;
use crate::vector::Mappers;

pub trait Transposes<R>: IntoIterator<Item=R> where
    R: IntoIterator,

{
    fn transpose(self) -> Matrix<R::Item> where
        Self: Sized,
        Self::IntoIter: Iterator<Item=R>,
        R::IntoIter: Iterator<Item=R::Item>
    {
        let rows_iterator = &mut self.into_iter();
        match rows_iterator.next() {
            None => { vec![] }
            Some(row) => {
                let mut columns = row.mapper(|x| vec![x]);
                rows_iterator.iterate(|row| {
                    row.indexed_iterate(|i, x| {
                        columns[i].push(x)
                    })
                });
                columns
            }
        }
    }
}

impl<R, M> Transposes<R> for M where
    M: IntoIterator<Item=R>,
    R: IntoIterator,
{}

pub fn transpose<M, R>(mx: M) -> Matrix<R::Item> where
    M: IntoIterator<Item=R>,
    R: IntoIterator,
    M::IntoIter: Iterator<Item=R>,
    R::IntoIter: Iterator<Item=R::Item>
{ mx.transpose() }

#[cfg(test)]
mod tests {
    // use spare::deco_matrix;

    use super::*;

    #[test]
    fn transpose_2d_vector() {
        let candidates = vec![
            vec![
                vec![1, 1, 1],
                vec![2, 2, 2],
                vec![3, 3, 3]
            ],
            vec![
                vec![1],
                vec![2],
                vec![3]
            ],
            vec![
                vec![1, 1, 1],
            ],
            vec![],
            vec![vec![]],
        ];
        for mx in &candidates {
            let result = transpose(mx);
            println!("{:?}", result);
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