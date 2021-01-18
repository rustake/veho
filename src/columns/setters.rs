use crate::vector::zipeach;

pub trait Setters<'a, T>: IntoIterator<Item=&'a mut Vec<T>> where
    T: 'a
{
    fn push_column<C>(self, column: C) where
        Self: Sized,
        C: IntoIterator<Item=T>
    {
        let iter = &mut self.into_iter();
        zipeach(iter, column, |row, x| row.push(x));
    }
}

impl<'a, T, M> Setters<'a, T> for M where
    M: IntoIterator<Item=&'a mut Vec<T>>,
    T: 'a
{}

// pub fn column<R, M>(matrix: M, c: usize) -> Vec<R::Item> where
//     M: IntoIterator<Item=R>,
//     R: IntoIterator,
//     R::IntoIter: Iterator<Item=R::Item>
// { matrix.column(c) }

#[cfg(test)]
mod tests {
    use crate::matrix::init;

    use super::*;

    #[test]
    fn test_push_column() {
        let mut matrix = init(3, 4, |i, j| ((i + 1) * 10 + (j + 1)) as i32);
        println!("{:?}", &matrix);
        (&mut matrix).push_column(vec![1, 1, 1]);
        println!("{:?}", &matrix);
    }
}