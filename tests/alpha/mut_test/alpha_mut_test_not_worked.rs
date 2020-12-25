// use std::slice::IterMut;
//
// pub fn mutate<'a, T: 'a, I: IntoIterator<Item=&'a mut T, IntoIter=IterMut<'a, T>>, F: FnMut(&'a mut T) -> ()>(
//     iterable: I, mut f: F,
// ) {
//     for x in &mut iterable.into_iter() {
//         f(&mut *x);
//     }
// }
//
// #[test]
// fn alpha_mut_test() {
//     let mut vec = vec![0, 1, 2, 3];
//     let lambda: fn(&mut i32) -> () = |x| *x += 1;
//     // vec.iter_mut().for_each(lambda);
//     mutate(vec, lambda);
//     println!("{:?}", vec);
// }