#[cfg(test)]
mod tests {
    use veho::vector::{mapper, iterate, mutate, Mapper};

    #[test]
    fn test_mapper_trait() {
        let vec = vec![1, 2, 3];
        println!("original: vec = {:?}", vec);
        let vec = &vec.mapper(|x| x + 1);
        println!("modified: vec = {:?}", vec);
    }

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