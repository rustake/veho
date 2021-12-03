use std::slice::IterMut;

#[test]
fn iter_mut_test() {
    let mut vec = vec![1, 2, 3];
    let lambda: fn(&mut i32) -> () = |i| *i += 1;
    vec.iter_mut().for_each(lambda);
    println!("{:?}", vec);
}

pub fn mutate2<T, F: FnMut(&mut T) -> ()>(
    it: IterMut<T>, f: F,
) {
    it.for_each(f);
}

#[test]
fn iter_mut_test_2() {
    let mut vec = vec![0, 1, 2, 3];
    mutate2(vec.iter_mut(), |x| *x += 1);
    println!("{:?}", vec);
}

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