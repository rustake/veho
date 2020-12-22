// use std::fmt::Debug;
//
// fn zip<A: Iterator + Debug, B: Iterator + Debug, C: Iterator + Debug>(&mut_test a: A, &b: B, &c: C) {
//     let zipper = a.into_iter().zip(b).zip(c).map(|((a, b), c)| (a, b, c));
//     for tup in zipper {
//         println!("{:#?}", tup);
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use crate::alpha::beta_zip_test::zip;
//
//     #[test]
//     fn test() {
//         let mut_test a = vec![1, 2, 3];
//         let b = vec![2, 4, 8];
//         let c = vec![5, 25, 125];
//         zip(&mut_test a, &b, &c);
//     }
// }