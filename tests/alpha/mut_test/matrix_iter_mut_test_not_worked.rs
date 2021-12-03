use std::slice::IterMut;

#[test]
fn iter_mut_test() {
    let mut matrix = vec![
        vec![1, 1, 1],
        vec![2, 2, 2],
        vec![3, 3, 3],
    ];
    let lambda: fn(&mut i32) -> () = |i| *i += 1;
    matrix.iter_mut().for_each(
        |row| row.iter_mut().for_each(
            lambda
        )
    );
    println!("{:?}", matrix);
}

// pub fn mutate2<'a, T: 'a, Row, F: FnMut(&mut T) -> ()>(matrix: IterMut<Row>, f: F) where
//     Row: IntoIterator<Item=&'a mut T, IntoIter=IterMut<'a, T>>
// {
//     matrix.for_each(|row| row.into_iter().for_each(f));
// }
//
// #[test]
// fn iter_mut_test_2() {
//     let mut matrix = vec![
//         vec![1, 1, 1],
//         vec![2, 2, 2],
//         vec![3, 3, 3]
//     ];
//     mutate2(matrix.iter_mut(), |x| *x += 1);
//     println!("{:?}", vec);
// }

pub fn mutate3<'a, T: 'a, I, F>(it: I, f: F) where
    I: IntoIterator<Item=&'a mut T, IntoIter=IterMut<'a, T>>,
    F: FnMut(&mut T) -> ()
{
    it.into_iter().for_each(f);
}

#[test]
fn iter_mut_test_3() {
    let mut vec = vec![0, 1, 2, 3];
    mutate3(&mut vec, |x| *x += 1);
    println!("{:?}", vec);
}

pub fn mutate4<'a, T: 'a, IT, FN>(it: IT, mut f: FN) where
    IT: IntoIterator<Item=&'a mut T, IntoIter=IterMut<'a, T>>,
    FN: FnMut(&mut T) -> ()
{
    for x in &mut it.into_iter() {
        let _ = &f(&mut *x);
    }
}

#[test]
fn iter_mut_test_4() {
    let mut vec = vec![0, 1, 2, 3];
    let lambda: fn(&mut i32) -> () = |x| *x += 1;
    // vec.iter_mut().for_each(lambda);
    mutate4(&mut vec, lambda);
    println!("{:?}", vec);

    let mut vec2 = [2, 4, 8, 16];
    mutate4(&mut vec2, lambda);
    println!("{:?}", vec2);
}