use std::ops::{Fn};


pub fn mapper<T, P, F: Fn(&T) -> P>(vec: &Vec<T>, f: F) -> Vec<P> {
    return vec.into_iter().map(f).collect::<Vec<P>>();
}

pub fn iterate<T, F: Fn(&T)>(vec: &Vec<T>, f: F) {
    for x in vec.iter() { f(x); }
}

pub fn mutate<T, F: Fn(&T) -> T>(vec: &mut Vec<T>, f: F) -> &Vec<T> {
    for x in vec.iter_mut() { *x = f(x); }
    return vec;
}

#[cfg(test)]
mod tests {
    use crate::vector::{mapper, iterate, mutate};

    #[test]
    fn test_mapper() {
        let vec = vec![1, 2, 3];
        println!("original: vec = {:?}", vec);
        let vec = mapper(&vec, |x| x + 1);
        println!("modified: vec = {:?}", vec);
    }

    #[test]
    fn test_iterate() {
        let mut vec = vec![1, 2, 3];
        iterate(&mut vec, |x| println!("{}", x + 1));
    }

    #[test]
    fn test_mutate() {
        let mut vec = vec![1, 2, 3];
        println!("original: vec = {:?}", vec);
        mutate(&mut vec, |x| x * 2);
        println!("modified: vec = {:?}", vec);
    }
}