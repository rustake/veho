use std::slice::IterMut;

pub fn mapper<I: IntoIterator, F: FnMut(I::Item) -> T, T>(iterable: I, f: F) -> Vec<T> {
    return iterable.into_iter().map(f).collect::<Vec<T>>();
}

pub fn iterate<I: IntoIterator, F: Fn(I::Item)>(iterable: I, f: F) {
    for x in iterable.into_iter() { f(x); }
}

pub fn mutate<'a, T: 'a, I, F>(it: I, mut f: F) where
    I: IntoIterator<Item=&'a mut T, IntoIter=IterMut<'a, T>>,
    F: FnMut(&mut T) -> ()
{
    for x in &mut it.into_iter() { &f(&mut *x); }
}

#[cfg(test)]
mod tests {
    use crate::iterable::mapper::{mapper, iterate, mutate};


    #[test]
    fn test_mapper() {
        let vec = [1, 2, 3];
        println!("original: vec = {:?}", vec);
        let vec = mapper(&vec, |x| x + 1);
        println!("modified: vec = {:?}", vec);
    }

    #[test]
    fn test_iterate() {
        let vec = vec![1, 2, 3];
        iterate(vec, |x| println!("{}", x + 1));
    }

    #[test]
    fn test_mutate() {
        let mut vec = vec![1, 2, 3];
        println!("original: vec = {:?}", vec);
        mutate(&mut vec, |x| *x *= 2);
        println!("modified: vec = {:?}", vec);
    }
}