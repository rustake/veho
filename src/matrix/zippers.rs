use crate::matrix::Matrix;
use crate::vector::{quazipeach as zipeach4, quazipper as zipper4, trizipeach as zipeach3, trizipper as zipper3};

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
            |((ra, rb), rc)| zipper3(ra, rb, rc, &mut f)
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
        .map(
            |(((ra, rb), rc), rd)| zipper4(ra, rb, rc, rd, &mut f)
        ).collect::<Matrix<T>>()
}

pub fn zipeach<RA, RB, MA, MB, T, F>(a: MA, b: MB, mut f: F)
    where RA: IntoIterator,
          RB: IntoIterator,
          MA: IntoIterator<Item=RA>,
          MB: IntoIterator<Item=RB>,
          F: FnMut(RA::Item, RB::Item)
{
    a.into_iter().zip(b).for_each(
        |(ra, rb)| ra.into_iter().zip(rb).for_each(
            |(a, b)| f(a, b)
        )
    )
}

pub fn trizipeach<RA, RB, RC, MA, MB, MC, T, F>(a: MA, b: MB, c: MC, mut f: F)
    where RA: IntoIterator,
          RB: IntoIterator,
          RC: IntoIterator,
          MA: IntoIterator<Item=RA>,
          MB: IntoIterator<Item=RB>,
          MC: IntoIterator<Item=RC>,
          F: FnMut(RA::Item, RB::Item, RC::Item)
{
    a.into_iter().zip(b).zip(c)
        .for_each(
            |((ra, rb), rc)| zipeach3(ra, rb, rc, &mut f)
        )
}

pub fn quazipeach<RA, RB, RC, RD, MA, MB, MC, MD, T, F>(a: MA, b: MB, c: MC, d: MD, mut f: F)
    where RA: IntoIterator,
          RB: IntoIterator,
          RC: IntoIterator,
          RD: IntoIterator,
          MA: IntoIterator<Item=RA>,
          MB: IntoIterator<Item=RB>,
          MC: IntoIterator<Item=RC>,
          MD: IntoIterator<Item=RD>,
          F: FnMut(RA::Item, RB::Item, RC::Item, RD::Item)
{
    a.into_iter().zip(b).zip(c).zip(d)
        .for_each(
            |(((ra, rb), rc), rd)| zipeach4(ra, rb, rc, rd, &mut f)
        )
}

#[cfg(test)]
mod tests {
    use crate::matrix::zippers::{quazipper, trizipper, zipper};

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
        println!(">> a = {:?}", a);
        println!(">> b = {:?}", b);
        println!(">> v = {:?}", v);
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
    //     println!(">> a = {:?}", a);
    //     println!(">> b = {:?}", b);
    //     println!(">> v = {:?}", v);
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
        println!(">> a = {:?}", a);
        println!(">> b = {:?}", b);
        println!(">> c = {:?}", c);
        println!(">> v = {:?}", v);
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
        println!(">> a = {:?}", a);
        println!(">> v = {:?}", v);
    }
}


