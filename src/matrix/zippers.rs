use crate::matrix::Matrix;

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

pub fn trizipper<RA, RB, RC, MA, MB, MC, T, F>(a: MA, b: MB, c: MC, mut f: F) -> Matrix<T>
    where RA: IntoIterator,
          RB: IntoIterator,
          RC: IntoIterator,
          MA: IntoIterator<Item=RA>,
          MB: IntoIterator<Item=RB>,
          MC: IntoIterator<Item=RC>,
          F: FnMut(RA::Item, RB::Item, RC::Item) -> T
{
    a.into_iter().zip(b).zip(c)
        .map(
            |((ra, rb), rc)| ra.into_iter().zip(rb).zip(rc).map(
                |((a, b), c)| f(a, b, c)
            ).collect::<Vec<T>>()
        ).collect::<Matrix<T>>()
}

pub fn quazipper<RA, RB, RC, RD, MA, MB, MC, MD, T, F>(a: MA, b: MB, c: MC, d: MD, mut f: F) -> Matrix<T>
    where RA: IntoIterator,
          RB: IntoIterator,
          RC: IntoIterator,
          RD: IntoIterator,
          MA: IntoIterator<Item=RA>,
          MB: IntoIterator<Item=RB>,
          MC: IntoIterator<Item=RC>,
          MD: IntoIterator<Item=RD>,
          F: FnMut(RA::Item, RB::Item, RC::Item, RD::Item) -> T
{
    a.into_iter().zip(b).zip(c).zip(d)
        .map(|(((ra, rb), rc), rd)|
            ra.into_iter().zip(rb).zip(rc).zip(rd).map(
                |(((a, b), c), d)| f(a, b, c, d)
            ).collect::<Vec<T>>()
        ).collect::<Matrix<T>>()
}

#[cfg(test)]
mod tests {
    use crate::matrix::zippers::{zipper, trizipper, quazipper};

    #[test]
    fn test_duo_zipper() {
        let a = vec![
            vec![10, 10, 10],
            vec![20, 20, 20],
            vec![30, 30, 30]
        ];
        let b = vec![
            vec![1, 2, 3],
            vec![1, 2, 3],
            vec![1, 2, 3]
        ];
        let v = zipper(&a, &b, |a, b| a + b);
        println!("a = {:?}", a);
        println!("b = {:?}", b);
        println!("v = {:?}", v);
    }

    //
    // #[test]
    // fn test_mut_zipper() {
    //     let mut a = vec![1, 2, 3];
    //     let b = vec![2, 4, 8];
    //     let v = zipper(&mut a, &b, |a, b| {
    //         *a *= b;
    //         return *a;
    //     });
    //     println!("a = {:?}", a);
    //     println!("b = {:?}", b);
    //     println!("v = {:?}", v);
    // }
    #[test]
    fn test_tri_zipper() {
        let mut a = vec![
            vec![10, 10, 10],
            vec![20, 20, 20],
            vec![30, 30, 30]
        ];
        let b = vec![
            vec![1, 2, 3],
            vec![1, 2, 3],
            vec![1, 2, 3]
        ];
        let c = vec![
            vec![1, 1, 1],
            vec![1, 1, 1],
            vec![1, 1, 1]
        ];
        let v = trizipper(&mut a, &b, &c, |a, b, c| {
            *a *= b + c;
            *a
        });
        println!("a = {:?}", a);
        println!("b = {:?}", b);
        println!("c = {:?}", c);
        println!("v = {:?}", v);
    }

    #[test]
    fn test_qua_zipper() {
        let mut a = vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
        let b = vec![vec![5, 25, 125], vec![5, 25, 125], vec![5, 25, 125]];
        let c = vec![vec![1, 3, 7], vec![1, 3, 7], vec![1, 3, 7]];
        let d = vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]];
        let v = quazipper(&mut a, &b, &c, &d,
                          |xa, xb, xc, xd| *xa += xb * (xc + xd),
        );
        println!("a = {:?}", a);
        println!("v = {:?}", v);
    }
}


