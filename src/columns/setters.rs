use crate::vector::zipeach;

pub trait Setters<'a, T: 'a>: IntoIterator<Item=&'a mut Vec<T>>
{
    fn push_column<C>(self, column: C) where
        Self: Sized,
        C: IntoIterator<Item=T>
    {
        let rows = &mut self.into_iter();
        zipeach(rows, column, |row, x| row.push(x));
    }
}

impl<'a, M, T: 'a> Setters<'a, T> for M where
    M: IntoIterator<Item=&'a mut Vec<T>>
{}

pub fn push_column<'a, M, C, T: 'a>(matrix: M, column: C) where
    M: IntoIterator<Item=&'a mut Vec<T>>,
    C: IntoIterator<Item=T>
{ matrix.push_column(column) }

#[cfg(test)]
mod tests {
    use crate::matrix::init;

    use super::*;

    #[test]
    fn test_push_column() {
        let mut matrix = init(3, 4, |i, j| ((i + 1) * 10 + (j + 1)) as i32);
        println!(">> {:?}", &matrix);
        (&mut matrix).push_column(vec![1, 2, 3]);
        println!(">> {:?}", &matrix);
    }
}