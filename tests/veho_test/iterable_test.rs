#[cfg(test)]
mod iterable_test {
    use veho::iterable::{mapper, zipper};

    #[test]
    fn mapper_test() {
        let a = [1, 2, 3];
        let v = mapper(a.iter(), |x| x + 1);
        println!("{:?}", v);
    }

    #[test]
    fn zipper_test() {
        let a = [1, 1, 1];
        let b = [1, 3, 7];
        let v = zipper(a.iter(), b.iter(), |x, y| x + y);
        println!("{:?}", v);
    }
}