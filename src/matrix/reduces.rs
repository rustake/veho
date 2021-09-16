use crate::vector::{
    mapflat as mapflat_vector,
    mapreduce as mapreduce_vector,
    reduce as reduce_vector,
};

pub trait Reduces<R>: IntoIterator<Item=R>
    where R: IntoIterator
{
    fn reduce<F>(self, sequence: F) -> Option<R::Item> where
        Self: Sized,
        Self::IntoIter: Iterator<Item=R>,
        R::IntoIter: Iterator<Item=R::Item>,
        F: FnMut(R::Item, R::Item) -> R::Item,
    { reduce_vector(self.into_iter().flatten(), sequence) }

    fn mapflat<T, J, F>(self, indicator: J, sequence: F) -> Option<T> where
        Self: Sized,
        Self::IntoIter: Iterator<Item=R>,
        R::IntoIter: Iterator<Item=R::Item>,
        J: FnMut(R::Item) -> T,
        F: FnMut(T, R::Item) -> T,
    { mapflat_vector(self.into_iter().flatten(), indicator, sequence) }

    fn mapreduce<T, J, F>(self, indicator: J, sequence: F) -> Option<T> where
        Self: Sized,
        Self::IntoIter: Iterator<Item=R>,
        R::IntoIter: Iterator<Item=R::Item>,
        J: FnMut(R::Item) -> T,
        F: FnMut(T, T) -> T,
    { mapreduce_vector(self.into_iter().flatten(), indicator, sequence) }
}

impl<R, M> Reduces<R> for M where
    M: IntoIterator<Item=R>,
    R: IntoIterator
{}

pub fn reduce<R, M, F>(matrix: M, f: F) -> Option<R::Item> where
    M: IntoIterator<Item=R>,
    R: IntoIterator,
    M::IntoIter: Iterator<Item=R>,
    R::IntoIter: Iterator<Item=R::Item>,
    F: FnMut(R::Item, R::Item) -> R::Item,
{ reduce_vector(matrix.into_iter().flatten(), f) }

pub fn mapflat<R, M, T, J, F>(matrix: M, indicator: J, sequence: F) -> Option<T> where
    M: IntoIterator<Item=R>,
    R: IntoIterator,
    M::IntoIter: Iterator<Item=R>,
    R::IntoIter: Iterator<Item=R::Item>,
    J: FnMut(R::Item) -> T,
    F: FnMut(T, R::Item) -> T,
{ mapflat_vector(matrix.into_iter().flatten(), indicator, sequence) }

pub fn mapreduce<R, M, T, J, F>(matrix: M, indicator: J, sequence: F) -> Option<T> where
    M: IntoIterator<Item=R>,
    R: IntoIterator,
    M::IntoIter: Iterator<Item=R>,
    R::IntoIter: Iterator<Item=R::Item>,
    J: FnMut(R::Item) -> T,
    F: FnMut(T, T) -> T,
{ mapreduce_vector(matrix.into_iter().flatten(), indicator, sequence) }

#[cfg(test)]
mod tests {
    use std::cmp::max;

    use super::*;

    #[test]
    fn test_reduce() {
        let sample = vec![
            vec![4, 2, 7],
            vec![7, 5, 9],
            vec![3, 0, 4],
        ];
        let result = sample.reduce(|a, b| max(a, b));
        println!("{}", result.unwrap());
    }
}