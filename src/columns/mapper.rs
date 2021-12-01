use crate::matrix::Utils;
use crate::vector::mapper as mapper_vector;

pub trait Mappers<R>: IntoIterator<Item=R>
    where R: IntoIterator
{
    fn mapper<P, F>(self, mut f: F) -> Vec<P> where
        Self: Sized,
        R::IntoIter: ExactSizeIterator<Item=R::Item>,
        F: FnMut(Vec<R::Item>) -> P
    { mapper_vector(self.transpose(), |col| f(col)) }
}

impl<R, M> Mappers<R> for M where
    M: IntoIterator<Item=R>,
    R: IntoIterator
{}

pub fn mapper<R, M, F, P>(matrix: M, f: F) -> Vec<P> where
    M: IntoIterator<Item=R>,
    R: IntoIterator,
    R::IntoIter: ExactSizeIterator<Item=R::Item>,
    F: FnMut(Vec<R::Item>) -> P
{ matrix.mapper(f) }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let matrix = vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]];
        let result = (&matrix).mapper(|col| mapper_vector(col, |x| { x + 1 }));
        println!(">> result = {:?}", result);
        println!(">> matrix = {:?}", matrix);
    }
}