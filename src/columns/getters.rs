use crate::vector::mapper as mapper_vector;

pub trait Getters<R>: IntoIterator<Item=R> where
    R: IntoIterator,
{
    fn column(self, c: usize) -> Vec<R::Item> where
        Self: Sized,
        R::IntoIter: Iterator<Item=R::Item>
    { mapper_vector(self, |row| row.into_iter().nth(c).unwrap()) }
}

impl<R, M> Getters<R> for M where
    M: IntoIterator<Item=R>,
    R: IntoIterator
{}

pub fn column<R, M>(matrix: M, c: usize) -> Vec<R::Item> where
    M: IntoIterator<Item=R>,
    R: IntoIterator,
    R::IntoIter: Iterator<Item=R::Item>
{ matrix.column(c) }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let matrix = vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]];
        let result = column(&matrix, 1);
        println!("result = {:?}", result);
        println!("matrix = {:?}", matrix);
    }
}