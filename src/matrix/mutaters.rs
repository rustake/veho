use std::slice::IterMut;

pub trait Mutaters<'a, R>: IntoIterator<Item=&'a mut R, IntoIter=IterMut<'a, R>> where
    Self::Item: IntoIterator<Item=&'a mut R::Item, IntoIter=IterMut<'a, R::Item>>,
    R: 'a + IntoIterator,
{
    fn mutate<F>(self, mut f: F) where
        Self: Sized,
        R: Sized,
        F: FnMut(&mut R::Item) -> ()
    {
        for row in &mut self.into_iter() {
            for x in &mut row.into_iter() {
                let _ = &f(&mut *x);
            }
        }
    }

    fn indexed_mutate<F>(self, mut f: F) where
        Self: Sized,
        R: Sized,
        F: FnMut(usize, usize, &mut R::Item) -> ()
    {
        for (i, row) in &mut self.into_iter().enumerate() {
            for (j, x) in &mut row.into_iter().enumerate() {
                let _ = &f(i, j, &mut *x);
            }
        }
    }
}

impl<'a, R, M: ?Sized> Mutaters<'a, R> for M where
    R: IntoIterator + 'a,
    M: IntoIterator<Item=&'a mut R, IntoIter=IterMut<'a, R>>,
    M::Item: IntoIterator<Item=&'a mut R::Item, IntoIter=IterMut<'a, R::Item>>,
{}

pub fn mutate<'a, R, M, F>(mx: M, f: F) where
    R: IntoIterator + 'a,
    M: IntoIterator<Item=&'a mut R, IntoIter=IterMut<'a, R>>,
    M::Item: IntoIterator<Item=&'a mut R::Item, IntoIter=IterMut<'a, R::Item>>,
    F: FnMut(&mut R::Item) -> ()
{ mx.mutate(f) } // for row in &mut mx.into_iter() { for x in &mut row.into_iter() { &f(&mut *x); } }

pub fn indexed_mutate<'a, R, M, F>(mx: M, f: F) where
    R: IntoIterator + 'a,
    M: IntoIterator<Item=&'a mut R, IntoIter=IterMut<'a, R>>,
    M::Item: IntoIterator<Item=&'a mut R::Item, IntoIter=IterMut<'a, R::Item>>,
    F: FnMut(usize, usize, &mut R::Item) -> ()
{ mx.indexed_mutate(f) } // for row in &mut mx.into_iter() { for x in &mut row.into_iter() { &f(&mut *x); } }

#[cfg(test)]
mod tests_mutaters {
    use crate::matrix::{mutate, Mutaters};

    #[test]
    fn test_vec_mutate() {
        let mut mx = vec![
            vec![1, 1, 1],
            vec![2, 2, 2],
            vec![3, 3, 3],
        ];
        // let f: fn(&mut i32) = |x| *x += 1;
        (&mut mx).mutate(|x| *x += 1);
        println!(">> {:?}", mx);
    }

    #[test]
    fn test_vec_indexed_mutate() {
        let mut mx = vec![
            vec![1, 1, 1],
            vec![2, 2, 2],
            vec![3, 3, 3],
        ];
        // let f: fn(&mut i32) = |x| *x += 1;
        (&mut mx).indexed_mutate(|i, j, x| *x += (i + j) as i32);
        println!(">> {:?}", mx);
    }

    #[test]
    fn test_vec_mutate2() {
        let mut mx = vec![
            vec![1, 1, 1],
            vec![2, 2, 2],
            vec![3, 3, 3],
        ];
        // let f: fn(&mut i32) = |x| *x += 1;
        mutate(&mut mx, |x| *x += 1);
        println!(">> {:?}", mx);
    }
}