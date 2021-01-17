pub trait Reduces: IntoIterator {
    fn reduce<F>(self, sequence: F) -> Option<Self::Item>
        where
            Self: Sized,
            Self::IntoIter: Iterator<Item=Self::Item>,
            F: FnMut(Self::Item, Self::Item) -> Self::Item
    {
        let mut iter = self.into_iter();
        iter.next().map(|ini| iter.fold(ini, sequence))
    }

    fn map_reduce<T, J, F>(self, mut indicator: J, mut sequence: F) -> Option<T>
        where
            Self: Sized,
            Self::IntoIter: Iterator<Item=Self::Item>,
            J: FnMut(Self::Item) -> T,
            F: FnMut(T, T) -> T
    {
        let mut iter = self.into_iter();
        iter.next().map(|ini| {
            iter.fold(indicator(ini), |a, b| sequence(a, indicator(b)))
        })
    }
}

impl<I> Reduces for I where
    I: IntoIterator,
{}

pub fn reduce<I, F>(vec: I, sequence: F) -> Option<I::Item>
    where
        I: IntoIterator,
        I: Sized,
        I::IntoIter: Iterator<Item=I::Item>,
        F: FnMut(I::Item, I::Item) -> I::Item
{ vec.reduce(sequence) }

pub fn map_reduce<I, T, J, F>(vec: I, indicator: J, sequence: F) -> Option<T>
    where
        I: IntoIterator,
        I: Sized,
        I::IntoIter: Iterator<Item=I::Item>,
        J: FnMut(I::Item) -> T,
        F: FnMut(T, T) -> T
{ vec.map_reduce(indicator, sequence) }

#[cfg(test)]
mod tests {
    use std::cmp::max;

    use super::*;

    #[test]
    fn test() {
        let vec = vec![1, 7, 4, 1];
        let some = vec.reduce(|a, b| max(a, b));
        println!("{}", some.unwrap());
    }

    #[test]
    fn test_map_reduce() {
        let vec = vec!["fo", "bar", "zene"];
        let some = vec.map_reduce(
            |x| x.len(),
            |a, b| max(a, b),
        );
        println!("{}", some.unwrap());
    }
}
