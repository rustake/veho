pub fn zipper<A, B, T, F>(a: A, b: B, mut f: F) -> Vec<T>
    where A: IntoIterator,
          B: IntoIterator,
          F: FnMut(A::Item, B::Item) -> T
{
    a.into_iter().zip(b)
        .map(|(a, b)| f(a, b))
        .collect::<Vec<T>>()
}

pub fn tri_zipper<A, B, C, T, F>(a: A, b: B, c: C, mut f: F) -> Vec<T>
    where A: IntoIterator,
          B: IntoIterator,
          C: IntoIterator,
          F: FnMut(A::Item, B::Item, C::Item) -> T
{
    a.into_iter().zip(b).zip(c)
        .map(|((a, b), c)| f(a, b, c))
        .collect::<Vec<T>>()
}

pub fn quo_zipper<A, B, C, D, T, F>(a: A, b: B, c: C, d: D, mut f: F) -> Vec<T>
    where A: IntoIterator,
          B: IntoIterator,
          C: IntoIterator,
          D: IntoIterator,
          F: FnMut(A::Item, B::Item, C::Item, D::Item) -> T
{
    a.into_iter().zip(b).zip(c).zip(d)
        .map(|(((a, b), c), d)| f(a, b, c, d))
        .collect::<Vec<T>>()
}

#[cfg(test)]
mod tests {
    use crate::iterable::zipper::{zipper, tri_zipper, quo_zipper};

    #[test]
    fn test_duo_zipper() {
        let a = vec![1, 2, 3];
        let b = vec![2, 4, 8];
        let v = zipper(&a, &b, |a, b| a * b);
        println!("a = {:?}", a);
        println!("b = {:?}", b);
        println!("v = {:?}", v);
    }

    #[test]
    fn test_tri_zipper() {
        let mut a = vec![1, 2, 3];
        let b = vec![2, 4, 8];
        let c = vec![1, 10, 100];
        let v = tri_zipper(&mut a, &b, &c, |a, b, c| *a *= b * c);
        println!("a = {:?}", a);
        println!("b = {:?}", b);
        println!("c = {:?}", c);
        println!("v = {:?}", v);
    }

    #[test]
    fn test_quo_zipper() {
        let mut a = vec![0, 0, 0];
        let b = vec![5, 25, 125];
        let c = vec![1, 3, 7];
        let d = vec![1, 1, 1];
        let v = quo_zipper(&mut a, &b, &c, &d,
                           |xa, xb, xc, xd| *xa += xb * (xc + xd),
        );
        println!("a = {:?}", a);
        println!("v = {:?}", v);
    }
}


