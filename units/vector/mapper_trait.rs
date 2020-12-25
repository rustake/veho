pub trait Mapper<T> {
    fn mapper<P, F: Fn(&T) -> P>(&self, f: F) -> Vec<P>;
    fn iterate<F: Fn(&T)>(&self, f: F);
    fn mutate<F: Fn(&T) -> T>(&mut self, f: F) -> &Vec<T>;
}

impl<T> Mapper<T> for Vec<T> {
    fn mapper<P, F: Fn(&T) -> P>(&self, f: F) -> Vec<P> {
        return self.into_iter().map(f).collect::<Vec<P>>();
    }
    fn iterate<F: Fn(&T)>(&self, f: F) {
        for x in self.iter() { f(x); }
    }
    fn mutate<F: Fn(&T) -> T>(&mut self, f: F) -> &Vec<T> {
        for x in self.iter_mut() { *x = f(x); }
        return self;
    }
}

#[cfg(test)]
mod tests {
    use crate::vector::mapper_trait::Mapper;

    #[test]
    fn test_mapper_trait() {
        let arr = vec![1, 2, 3];
        println!("original: vec = {:?}", arr);
        let vec = arr.mapper(|x| x + 1);
        println!("original: arr = {:?}", arr);
        println!("mapped: vec = {:?}", vec);
    }
}