use std::slice::IterMut;

pub fn mapper<I, F, T>(iterable: I, mut f: F) -> Vec<T> where
    I: IntoIterator,
    F: FnMut(usize, I::Item) -> T {
    return iterable.into_iter().enumerate()
        .map(|(i, x)| f(i, x))
        .collect::<Vec<T>>();
}

pub fn iterate<I, F>(iterable: I, f: F) where
    I: IntoIterator,
    F: Fn(usize, I::Item) {
    for (i, x) in iterable.into_iter().enumerate() { f(i, x); }
}

pub fn mutate<'a, T: 'a, I, F>(it: I, mut f: F) where
    I: IntoIterator<Item=&'a mut T, IntoIter=IterMut<'a, T>>,
    F: FnMut(usize, &mut T) -> () {
    for (i, x) in &mut it.into_iter().enumerate() { &f(i, &mut *x); }
}

#[cfg(test)]
mod tests {
    use crate::iterable::enumerate::mapper::{mapper, iterate, mutate};


    #[test]
    fn test_mapper() {
        let vec = [1, 2, 3];
        println!("original: vec = {:?}", vec);
        let vec = mapper(&vec, |i, x| x + i as i32);
        println!("modified: vec = {:?}", vec);
    }

    #[test]
    fn test_iterate() {
        let vec = vec![1, 2, 3];
        iterate(vec, |i, x| println!("[{}] ({})", i, x + 1));
    }

    #[test]
    fn test_mutate() {
        let mut vec = vec![1, 2, 3];
        println!("original: vec = {:?}", vec);
        mutate(&mut vec, |i, x| *x *= i * 2);
        println!("modified: vec = {:?}", vec);
    }
}