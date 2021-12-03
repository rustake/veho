use crate::matrix::Utils;
use crate::vector::mapper as mapper_vector;

pub trait Mappers<R>: IntoIterator<Item=R>
    where R: IntoIterator
{
    fn map_columns<P, F>(self, mut fn_on_column: F) -> Vec<P> where
        Self: Sized,
        R::IntoIter: ExactSizeIterator<Item=R::Item>,
        F: FnMut(Vec<R::Item>) -> P
    { mapper_vector(self.transpose(), |col| fn_on_column(col)) }
}

impl<R, M> Mappers<R> for M where
    M: IntoIterator<Item=R>,
    R: IntoIterator
{}

pub fn map_columns<R, M, F, P>(matrix: M, f: F) -> Vec<P> where
    M: IntoIterator<Item=R>,
    R: IntoIterator,
    R::IntoIter: ExactSizeIterator<Item=R::Item>,
    F: FnMut(Vec<R::Item>) -> P
{ matrix.map_columns(f) }

#[deprecated(since = "0.0.20", note = "please use `map_columns` instead")]
pub fn mapper<R, M, F, P>(matrix: M, f: F) -> Vec<P> where
    M: IntoIterator<Item=R>,
    R: IntoIterator,
    R::IntoIter: ExactSizeIterator<Item=R::Item>,
    F: FnMut(Vec<R::Item>) -> P
{ matrix.map_columns(f) }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let matrix = vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]];
        let result = (&matrix).map_columns(|col| mapper_vector(col, |x| { x + 1 }));
        println!(">> result = {:?}", result);
        println!(">> matrix = {:?}", matrix);
    }
}