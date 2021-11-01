use std::slice::IterMut;

pub trait Mappers: IntoIterator {
    fn mapper<F, T>(self, f: F) -> Vec<T> where
        Self: Sized,
        F: FnMut(Self::Item) -> T
    { self.into_iter().map(f).collect::<Vec<T>>() }

    fn iterate<F>(self, mut f: F) where
        Self: Sized,
        F: FnMut(Self::Item)
    { for x in self.into_iter() { f(x); } }

    fn mutate<'a, T: 'a, F>(self, mut f: F) where
        Self: IntoIterator<Item=&'a mut T, IntoIter=IterMut<'a, T>> + Sized,
        F: FnMut(&mut T) -> ()
    { for x in &mut self.into_iter() { let _ = &f(&mut *x); } }

    fn indexed_mapper<F, T>(self, mut f: F) -> Vec<T> where
        Self: Sized,
        F: FnMut(usize, Self::Item) -> T
    { self.into_iter().enumerate().map(|(i, x)| f(i, x)).collect::<Vec<T>>() }

    fn indexed_iterate<F>(self, mut f: F) where
        Self: Sized,
        F: FnMut(usize, Self::Item)
    { for (i, x) in self.into_iter().enumerate() { f(i, x); } }

    fn indexed_mutate<'a, T: 'a, F>(self, mut f: F) where
        Self: IntoIterator<Item=&'a mut T, IntoIter=IterMut<'a, T>> + Sized,
        F: FnMut(usize, &mut T) -> ()
    { for (i, x) in &mut self.into_iter().enumerate() { let _ = &f(i, &mut *x); } }
}

impl<T: ?Sized> Mappers for T where T: IntoIterator {}

pub fn mapper<I, F, T>(it: I, f: F) -> Vec<T> where
    I: IntoIterator,
    F: FnMut(I::Item) -> T
{ it.mapper(f) }

pub fn iterate<I, F>(it: I, f: F) where
    I: IntoIterator,
    F: FnMut(I::Item)
{ it.iterate(f) }

pub fn mutate<'a, T: 'a, I, F>(it: I, f: F) where
    I: IntoIterator<Item=&'a mut T, IntoIter=IterMut<'a, T>>,
    F: FnMut(&mut T) -> ()
{ it.mutate(f) }

pub fn indexed_mapper<I, F, T>(it: I, f: F) -> Vec<T> where
    I: IntoIterator,
    F: FnMut(usize, I::Item) -> T
{ it.indexed_mapper(f) }

pub fn indexed_iterate<I, F>(it: I, f: F) where
    I: IntoIterator,
    F: FnMut(usize, I::Item)
{ it.indexed_iterate(f) }

pub fn indexed_mutate<'a, T: 'a, I, F>(it: I, f: F) where
    I: IntoIterator<Item=&'a mut T, IntoIter=IterMut<'a, T>>,
    F: FnMut(usize, &mut T) -> ()
{ it.indexed_mutate(f) }

#[cfg(test)]
mod mappers_tests {
    use crate::vector::{iterate, mapper, mutate};

    use super::Mappers;

    #[test]
    fn test_mapper() {
        let original: Vec<i32> = vec![1, 2, 3];
        println!("original: vec = {:?}", original);
        let mapped = (&original).mapper(|x| x + 1);
        // let vec = mapper(vec.iter().into_iter(), |x| x + 1);
        println!("modified: vec = {:?}", mapped);
        println!("original: vec = {:?}", original);
    }

    #[test]
    fn test_iterate() {
        let vec = vec![1, 2, 3];
        vec.iterate(|x| println!("{}", x + 1));
    }

    #[test]
    fn test_iterate_fn_mut() {
        let mut text = "_".to_owned();
        let vec = vec![1, 2, 3];
        vec.iterate(|x| {
            let _ = &mut text.push_str(&x.to_string());
            println!("{}", x + 1)
        });
        println!("{}", text);
    }

    #[test]
    fn test_iterate_mutate() {
        let mut vec = vec![1, 2, 3];
        (&mut vec).iterate(|x| *x += 1);
        println!("modified: vec = {:?}", vec);
    }

    #[test]
    fn test_mutate() {
        let mut vec = vec![1, 2, 3];
        println!("original: vec = {:?}", vec);
        (&mut vec).mutate(|x| *x *= 2);
        println!("modified: vec = {:?}", vec);
    }

    #[test]
    fn test_mapper_func() {
        let vec = [1, 2, 3];
        println!("original: vec = {:?}", vec);
        let vec2 = mapper(&vec, |x| x + 1);
        println!("modified: vec2 = {:?}", vec2);
        println!("original: vec = {:?}", vec);
    }

    #[test]
    fn test_iterate_func() {
        let vec = vec![1, 2, 3];
        iterate(vec, |x| println!("{}", x + 1));
    }

    #[test]
    fn test_mutate_func() {
        let mut vec = vec![1, 2, 3];
        println!("original: vec = {:?}", vec);
        mutate(&mut vec, |x| *x *= 2);
        println!("modified: vec = {:?}", vec);
    }
}

#[cfg(test)]
mod indexed_mappers_tests {
    use super::Mappers;

    #[test]
    fn test_mapper() {
        let vec = vec![1, 2, 3];
        println!("original: vec = {:?}", vec);
        let vec = vec.indexed_mapper(|i, x| x + i);
        // let vec = mapper(vec.iter().into_iter(), |x| x + 1);
        println!("modified: vec = {:?}", vec);
    }

    #[test]
    fn test_iterate() {
        let vec = vec![1, 2, 3];
        vec.indexed_iterate(|i, x| println!("[{}] ({})", i, x));
    }

    #[test]
    fn test_mutate() {
        let mut vec = vec![1, 2, 3];
        println!("original: vec = {:?}", vec);
        (&mut vec).indexed_mutate(|i, x| *x *= i);
        println!("modified: vec = {:?}", vec);
    }
}

#[cfg(test)]
mod indexed_mappers_func_tests {
    use crate::vector::mappers::{indexed_iterate, indexed_mapper, indexed_mutate};

    #[test]
    fn test_mapper() {
        let vec = [1, 2, 3];
        println!("original: vec = {:?}", vec);
        let vec = indexed_mapper(&vec, |i, x| x + i as i32);
        println!("modified: vec = {:?}", vec);
    }

    #[test]
    fn test_iterate() {
        let vec = vec![1, 2, 3];
        indexed_iterate(vec, |i, x| println!("[{}] ({})", i, x + 1));
    }

    #[test]
    fn test_mutate() {
        let mut vec = vec![1, 2, 3];
        println!("original: vec = {:?}", vec);
        indexed_mutate(&mut vec, |i, x| *x *= i * 2);
        println!("modified: vec = {:?}", vec);
    }
}