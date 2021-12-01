use std::ops::Index;

pub trait RefColumns<'a, R>: IntoIterator<Item=&'a R> where
    R: 'a + Index<usize>
{
    fn column(self, c: usize) -> Vec<&'a R::Output> where
        Self: Sized
    {
        self.into_iter()
            .map(|row| &row[c])
            .collect::<Vec<&R::Output>>()
    }

    fn clone_column(self, c: usize) -> Vec<R::Output> where
        R::Output: Clone,
        Self: Sized
    {
        self.into_iter()
            .map(|row| row[c].clone())
            .collect::<Vec<R::Output>>()
    }
}

impl<'a, R, M> RefColumns<'a, R> for M where
    M: IntoIterator<Item=&'a R>,
    R: 'a + Index<usize>
{}

pub fn column<'a, R, M>(matrix: M, c: usize) -> Vec<&'a R::Output> where
    M: IntoIterator<Item=&'a R>,
    R: 'a + Index<usize>
{ matrix.column(c) }

pub fn clone_column<'a, R, M>(matrix: M, c: usize) -> Vec<R::Output> where
    M: IntoIterator<Item=&'a R>,
    R: 'a + Index<usize>,
    R::Output: Clone
{ matrix.clone_column(c) }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ref_column() {
        let matrix = vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]];
        let row = &matrix[1];
        println!(">> {:?}", row);
        let col = (&matrix).column(1);
        println!(">> {:?}", col);
    }

    #[test]
    fn test_clone_column() {
        let matrix = vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]];
        let row = &matrix[1];
        println!(">> {:?}", row);
        let col = &matrix.clone_column(1);
        println!(">> {:?}", col);
    }
}