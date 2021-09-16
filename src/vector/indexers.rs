pub trait Indexers: IntoIterator {
    fn index_of(self, item: Self::Item) -> Option<usize> where Self: Sized,
                                                               Self::IntoIter: Iterator<Item=Self::Item>,
                                                               Self::Item: PartialEq
    { self.into_iter().position(|x| match x == item { true => true, _ => false }) }
}

impl<I> Indexers for I where
    I: IntoIterator
{}

pub fn index_of<I>(vec: I, item: I::Item) -> Option<usize> where I: IntoIterator,
                                                                 I: Sized,
                                                                 I::IntoIter: Iterator<Item=I::Item>,
                                                                 I::Item: PartialEq
{ vec.index_of(item) }

#[cfg(test)]
mod tests {
    use crate::vector::indexers::{index_of, Indexers};
    use crate::vector::Mappers;

    fn to_isize(val: Option<usize>) -> isize {
        match val {
            Some(n) => n as isize,
            None => -1,
        }
    }

    #[test]
    fn test_crostab_simplified() {
        let years = vec!["2004", "1984", "1964"].mapper(|x| x.to_string());
        let year_label = "1984";
        let year_index = years.iter().index_of(&year_label.to_string());
        println!("years.index_of( {} ) = {}", year_label, to_isize(year_index));

        let year_label = "2024";
        let year_index = index_of(years.iter(), &year_label.to_string());
        println!("years.index_of( {} ) = {}", year_label, to_isize(year_index));

        let words = vec!["foo", "bar", "zen"].mapper(|x| x.to_string());
        let word_label = "voo";
        let word_index = words.iter().index_of(&word_label.to_string());
        println!("years.index_of( {} ) = {}", word_label, to_isize(word_index));
    }
}