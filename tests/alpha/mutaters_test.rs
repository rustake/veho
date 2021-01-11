use std::slice::IterMut;

pub trait Mutaters<'a, T, R>: IntoIterator<Item=&'a mut R, IntoIter=IterMut<'a, R>> where
    T: 'a,
    R: 'a + IntoIterator<Item=&'a mut T, IntoIter=IterMut<'a, T>>,
{
    fn mutate<F>(self, mut f: F) where
        Self: Sized,
        R: Sized,
        F: FnMut(&mut T) -> ()
    {
        for row in &mut self.into_iter() {
            for x in &mut row.into_iter() {
                &f(x);
            }
        }
    }
}

impl<'a, T, R, M: ?Sized> Mutaters<'a, T, R> for M where
    T: 'a,
    R: 'a + IntoIterator<Item=&'a mut T, IntoIter=IterMut<'a, T>>,
    M: IntoIterator<Item=&'a mut R, IntoIter=IterMut<'a, R>>,
{}

// impl<'a, T, R, M: ?Sized> Mutaters<'a, T, R> for M where
//     T: 'a,
//     R: IntoIterator + 'a,
//     M: IntoIterator<Item=&'a mut R, IntoIter=IterMut<'a, R>>,
//     M::Item: IntoIterator<Item=&'a mut R::Item, IntoIter=IterMut<'a, T>>,
// {}

#[cfg(test)]
mod tests_mutaters {
    use super::*;

    #[test]
    fn test_vec_mutate() {
        let mut mx = vec![
            vec![1, 1, 1],
            vec![2, 2, 2],
            vec![3, 3, 3]
        ];
        // let f: fn(&mut i32) = |x| *x += 1;
        (&mut mx).mutate(|x| *x += 1);
        println!("{:?}", mx);
    }
}