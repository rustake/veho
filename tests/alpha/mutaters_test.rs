use veho::matrix::Matrix;

pub fn zipper<RA, RB, MA, MB, T, F>(a: MA, b: MB, mut f: F) -> Matrix<T>
    where RA: IntoIterator,
          RB: IntoIterator,
          MA: IntoIterator<Item=RA>,
          MB: IntoIterator<Item=RB>,
          F: FnMut(RA::Item, RB::Item) -> T
{
    a.into_iter().zip(b).map(
        |(ra, rb)| ra.into_iter().zip(rb).map(
            |(a, b)| f(a, b)
        ).collect::<Vec<T>>()
    ).collect::<Matrix<T>>()
}

pub trait Mutaters<R>: IntoIterator<Item=R> where
    R: IntoIterator,
{
    fn mutate<F>(self, mut f: F) where
        Self: Sized,
        R: Sized,
        F: FnMut(R::Item) -> ()
    {
        for row in self.into_iter() {
            for x in row.into_iter() {
                f(x);
            }
        }
    }
}

impl<R, M: ?Sized> Mutaters<R> for M where
    R: IntoIterator,
    M: IntoIterator<Item=R>,
{}

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