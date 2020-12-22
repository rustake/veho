// struct Zip<T> {
//     t: T
// }
//
// pub fn multizip<T, U>(t: U) -> Zip<T>
//     where Zip<T>: From<U>,
//           Zip<T>: Iterator,
// {
//     Zip::from(t)
// }

pub fn zipper<A, B, P, F: FnMut(&A, &B) -> P>(a: &Vec<A>, b: &Vec<B>, mut f: F) -> Vec<P> {
    return a.iter().zip(b)
        .map(|(x, y)| f(x, y))
        .collect();
}

pub fn tri_zipper<A, B, C, P, F: FnMut(&A, &B, &C) -> P>
(a: &Vec<A>, b: &Vec<B>, c: &Vec<C>, mut f: F) -> Vec<P> {
    return a.iter().zip(b).zip(c)
        .map(|((x, y), z)| f(x, y, z))
        .collect();
}

pub fn quo_zipper<A, B, C, D, P, F: FnMut(&A, &B, &C, &D) -> P>
(a: &Vec<A>, b: &Vec<B>, c: &Vec<C>, d: &Vec<D>, mut f: F) -> Vec<P> {
    return a.iter().zip(b).zip(c).zip(d)
        .map(|(((xa, xb), xc), xd)| f(xa, xb, xc, xd))
        .collect();
}

#[cfg(test)]
mod tests {
    use crate::vector::zipper::{zipper, tri_zipper, quo_zipper};

    #[test]
    fn test_zipper() {
        let a = vec![5, 25, 125];
        let b = vec![2, 4, 8];
        let c = zipper(&a, &b, |x, y| x * y);
        println!("{:?}", c);
    }

    #[test]
    fn test_tri_zipper() {
        let a = vec![5, 25, 125];
        let b = vec![2, 4, 8];
        let c = vec![1, 2, 3];
        let d = tri_zipper(&a, &b, &c, |x, y, z| x * y * z);
        println!("{:?}", d);
    }

    #[test]
    fn test_quo_zipper() {
        let a = vec![0, 0, 0];
        let b = vec![5, 25, 125];
        let c = vec![1, 3, 7];
        let d = vec![1, 1, 1];
        let v = quo_zipper(&a, &b, &c, &d,
                           |xa, xb, xc, xd| xa + xb * (xc + xd),
        );
        println!("a = {:?}", a);
        println!("v = {:?}", v);
    }

    #[test]
    fn test_iterator() {
        let vec = vec![1, 2, 3, 4, 5];
        let iter = vec.iter();
        for x in iter {
            println!("{}", x);
        }
        // println!("{:?}", vec);
        let another = vec.iter().map(|x| *x).collect::<Vec<i32>>();
        println!("{:?}", another);
    }
}