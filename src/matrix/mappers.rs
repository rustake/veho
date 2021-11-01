use crate::matrix::Matrix;

// trait MatrixTrait<T> =
// IntoIterator where
// Self::Item: IntoIterator;

pub trait Mappers<R>: IntoIterator<Item=R>
    where R: IntoIterator
{
    fn mapper<P, F>(self, mut f: F) -> Matrix<P> where
        Self: Sized,
        F: FnMut(R::Item) -> P,
    {
        self.into_iter().map(
            |row| row.into_iter().map(
                |x| f(x)
            ).collect()
        ).collect()
    }

    fn iterate<F>(self, mut f: F) where
        Self: Sized,
        F: FnMut(R::Item)
    {
        self.into_iter().for_each(
            |row| row.into_iter().for_each(
                |x| f(x)
            )
        )
    }

    fn indexed_mapper<P, F>(self, mut f: F) -> Matrix<P> where
        Self: Sized,
        F: FnMut(usize, usize, R::Item) -> P,
    {
        self.into_iter().enumerate().map(
            |(i, row)| row.into_iter().enumerate().map(
                |(j, v)| f(i, j, v)
            ).collect()
        ).collect()
    }

    fn indexed_iterate<F>(self, mut f: F) where
        Self: Sized,
        F: FnMut(usize, usize, R::Item)
    {
        self.into_iter().enumerate().for_each(
            |(i, row)| row.into_iter().enumerate().for_each(
                |(j, v)| f(i, j, v)
            )
        )
    }
}


impl<R, M> Mappers<R> for M where
    R: IntoIterator,
    M: IntoIterator<Item=R> + ?Sized
{}


pub fn mapper<M, R, P, F>(mx: M, f: F) -> Matrix<P> where
    R: IntoIterator,
    M: IntoIterator<Item=R>,
    F: FnMut(R::Item) -> P,
{ mx.mapper(f) }

pub fn iterate<M, R, F>(mx: M, f: F) where
    R: IntoIterator,
    M: IntoIterator<Item=R>,
    F: FnMut(R::Item)
{ mx.iterate(f) }


pub fn indexed_mapper<M, R, P, F>(mx: M, f: F) -> Matrix<P> where
    R: IntoIterator,
    M: IntoIterator<Item=R>,
    F: FnMut(usize, usize, R::Item) -> P,
{ mx.indexed_mapper(f) }

pub fn indexed_iterate<M, R, F>(mx: M, f: F) where
    R: IntoIterator,
    M: IntoIterator<Item=R>,
    F: FnMut(usize, usize, R::Item)
{ mx.indexed_iterate(f) }


#[cfg(test)]
mod tests_mappers {
    use crate::matrix::Mappers;

    #[test]
    fn test_vec_mapper() {
        let mx = vec![
            vec![1, 1, 1],
            vec![2, 2, 2],
            vec![3, 3, 3],
        ];
        let result = (&mx).mapper(|x| x + 1);
        println!("{:?}", result);
    }

    #[test]
    fn test_vec_iterate() {
        let mut mx = vec![
            vec![1, 1, 1],
            vec![2, 2, 2],
            vec![3, 3, 3],
        ];
        let result = (&mut mx).iterate(|x| {
            *x += 1;
            print!("{}, ", x)
        });
        println!("{:?}", result);
        println!("{:?}", mx);
    }

    #[test]
    fn test_vec_mutate() {
        let mut mx = vec![
            vec![1, 1, 1],
            vec![2, 2, 2],
            vec![3, 3, 3],
        ];
        let mut text = "_".to_owned();
        (&mut mx).iterate(|x| {
            let _ = &mut text.push_str(&format!("#{:0>6X}", x));
            *x += 1
        });
        println!("{:?}", mx);
    }

    #[test]
    fn test_vec_indexed_mapper() {
        let mx = vec![
            vec![1, 1, 1],
            vec![2, 2, 2],
            vec![3, 3, 3],
        ];
        let result = mx.indexed_mapper(|i, j, x| (i, j, x));
        println!("{:?}", result);
    }

    #[test]
    fn test_vec_indexed_iterate() {
        let mx = vec![
            vec![1, 1, 1],
            vec![2, 2, 2],
            vec![3, 3, 3],
        ];
        mx.indexed_iterate(|i, j, x| print!("({}, {}) [{}], ", i, j, x + 1));
    }
}


#[cfg(test)]
mod arr_matrix_mappers_tests {
    use crate::matrix::Mappers;

    #[test]
    fn test_arr_mapper() {
        let mx = [
            [1, 1, 1],
            [2, 2, 2],
            [3, 3, 3]
        ];
        let result = mx.iter().mapper(|x| *x + 1);
        println!("{:?}", result);
    }
}