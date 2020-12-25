use crate::matrix::Matrix;
use std::slice::IterMut;


pub trait Mappers<R>: IntoIterator<Item=R>
    where R: IntoIterator
{
    fn mapper<P, F>(self, f: F) -> Matrix<P> where
        Self: Sized,
        F: Fn(R::Item) -> P,
    {
        return self.into_iter().map(
            |row| row.into_iter().map(
                &f
            ).collect()
        ).collect();
    }

    fn iterate<F>(self, f: F) where
        Self: Sized,
        F: Fn(R::Item)
    {
        self.into_iter().for_each(
            |row| row.into_iter().for_each(
                &f
            )
        )
    }

    fn indexed_mapper<P, F>(self, f: F) -> Matrix<P> where
        Self: Sized,
        F: Fn(usize, usize, R::Item) -> P,
    {
        return self.into_iter().enumerate().map(
            |(i, row)| row.into_iter().enumerate().map(
                |(j, v)| f(i, j, v)
            ).collect()
        ).collect();
    }

    fn indexed_iterate<F>(self, f: F) where
        Self: Sized,
        F: Fn(usize, usize, R::Item)
    {
        self.into_iter().enumerate().for_each(
            |(i, row)| row.into_iter().enumerate().for_each(
                |(j, v)| f(i, j, v)
            )
        )
    }
}

pub trait Mutaters<'a, R>: IntoIterator<Item=&'a mut R, IntoIter=IterMut<'a, R>>
    where R: IntoIterator + 'a
{
    fn mutate<F>(self, mut f: F) where
        Self: IntoIterator<Item=&'a mut R, IntoIter=IterMut<'a, R>> + Sized,
        Self::Item: IntoIterator<Item=&'a mut R::Item, IntoIter=IterMut<'a, R::Item>> + Sized,
        F: FnMut(&mut R::Item) -> ()
    {
        for row in &mut self.into_iter() {
            for x in &mut row.into_iter() {
                &f(&mut *x);
            }
        }
    }

    fn indexed_mutate<F>(self, mut f: F) where
        Self: IntoIterator<Item=&'a mut R, IntoIter=IterMut<'a, R>> + Sized,
        Self::Item: IntoIterator<Item=&'a mut R::Item, IntoIter=IterMut<'a, R::Item>> + Sized,
        F: FnMut(usize, usize, &mut R::Item) -> ()
    {
        for (i, row) in &mut self.into_iter().enumerate() {
            for (j, x) in &mut row.into_iter().enumerate() {
                &f(i, j, &mut *x);
            }
        }
    }
}

impl<R, M: ?Sized> Mappers<R> for M where
    R: IntoIterator,
    M: IntoIterator<Item=R>,
{}

impl<'a, R, M: ?Sized> Mutaters<'a, R> for M where
    R: IntoIterator + 'a,
    M: IntoIterator<Item=&'a mut R, IntoIter=IterMut<'a, R>>,
    M::Item: IntoIterator<Item=&'a mut R::Item, IntoIter=IterMut<'a, R::Item>>,
{}

pub fn mapper<M, R, P, F>(mx: M, f: F) -> Matrix<P> where
    R: IntoIterator,
    M: IntoIterator<Item=R>,
    F: Fn(R::Item) -> P,
{ mx.mapper(f) }

pub fn iterate<M, R, F>(mx: M, f: F) where
    R: IntoIterator,
    M: IntoIterator<Item=R>,
    F: Fn(R::Item)
{ mx.iterate(f) }

pub fn mutate<'a, R, M, F>(mx: M, f: F) where
    R: IntoIterator + 'a,
    M: IntoIterator<Item=&'a mut R, IntoIter=IterMut<'a, R>>,
    M::Item: IntoIterator<Item=&'a mut R::Item, IntoIter=IterMut<'a, R::Item>>,
    F: FnMut(&mut R::Item) -> ()
{ mx.mutate(f) } // for row in &mut mx.into_iter() { for x in &mut row.into_iter() { &f(&mut *x); } }

pub fn indexed_mapper<M, R, P, F>(mx: M, f: F) -> Matrix<P> where
    R: IntoIterator,
    M: IntoIterator<Item=R>,
    F: Fn(usize, usize, R::Item) -> P,
{ mx.indexed_mapper(f) }

pub fn indexed_iterate<M, R, F>(mx: M, f: F) where
    R: IntoIterator,
    M: IntoIterator<Item=R>,
    F: Fn(usize, usize, R::Item)
{ mx.indexed_iterate(f) }

pub fn indexed_mutate<'a, R, M, F>(mx: M, f: F) where
    R: IntoIterator + 'a,
    M: IntoIterator<Item=&'a mut R, IntoIter=IterMut<'a, R>>,
    M::Item: IntoIterator<Item=&'a mut R::Item, IntoIter=IterMut<'a, R::Item>>,
    F: FnMut(usize, usize, &mut R::Item) -> ()
{ mx.indexed_mutate(f) } // for row in &mut mx.into_iter() { for x in &mut row.into_iter() { &f(&mut *x); } }


#[cfg(test)]
mod tests_mappers {
    use crate::matrix::Mappers;

    #[test]
    fn test_vec_mapper() {
        let mx = vec![
            vec![1, 1, 1],
            vec![2, 2, 2],
            vec![3, 3, 3]
        ];
        let result = mx.mapper(|x| x + 1);
        println!("{:?}", result);
    }

    #[test]
    fn test_vec_iterate() {
        let mx = vec![
            vec![1, 1, 1],
            vec![2, 2, 2],
            vec![3, 3, 3]
        ];
        let result = mx.iterate(|x| print!("{}, ", x + 1));
        println!("{:?}", result);
    }

    #[test]
    fn test_vec_mutate() {
        let mut mx = vec![
            vec![1, 1, 1],
            vec![2, 2, 2],
            vec![3, 3, 3]
        ];
        (&mut mx).iterate(|x| *x += 1);
        println!("{:?}", mx);
    }

    #[test]
    fn test_vec_indexed_mapper() {
        let mx = vec![
            vec![1, 1, 1],
            vec![2, 2, 2],
            vec![3, 3, 3]
        ];
        let result = mx.indexed_mapper(|i, j, x| (i, j, x));
        println!("{:?}", result);
    }

    #[test]
    fn test_vec_indexed_iterate() {
        let mx = vec![
            vec![1, 1, 1],
            vec![2, 2, 2],
            vec![3, 3, 3]
        ];
        mx.indexed_iterate(|i, j, x| print!("({}, {}) [{}], ", i, j, x + 1));
    }
}

#[cfg(test)]
mod tests_mutaters {
    use crate::matrix::{Mutaters, mutate};

    #[test]
    fn test_vec_mutate() {
        let mut mx = vec![
            vec![1, 1, 1],
            vec![2, 2, 2],
            vec![3, 3, 3]
        ];
        // let f: fn(&mut i32) = |x| *x += 1;
        (&mut mx).mutate(|x| *x += 1);
        println!("{:?}", mx);
    }

    #[test]
    fn test_vec_indexed_mutate() {
        let mut mx = vec![
            vec![1, 1, 1],
            vec![2, 2, 2],
            vec![3, 3, 3]
        ];
        // let f: fn(&mut i32) = |x| *x += 1;
        (&mut mx).indexed_mutate(|i, j, x| *x += (i + j) as i32);
        println!("{:?}", mx);
    }

    #[test]
    fn test_vec_mutate2() {
        let mut mx = vec![
            vec![1, 1, 1],
            vec![2, 2, 2],
            vec![3, 3, 3]
        ];
        // let f: fn(&mut i32) = |x| *x += 1;
        mutate(&mut mx, |x| *x += 1);
        println!("{:?}", mx);
    }
}

#[cfg(test)]
mod arr_matrix_mappers_tests {
    use crate::matrix::Mappers;

    #[test]
    fn test_arr_mapper() {
        let mx = [
            [1, 1, 1],
            [2, 2, 2],
            [3, 3, 3]
        ];
        let result = mx.iter().mapper(|x| *x + 1);
        println!("{:?}", result);
    }
}