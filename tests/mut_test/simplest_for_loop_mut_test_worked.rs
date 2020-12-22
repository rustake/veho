#[test]
fn test_mutate() {
    let mut vec = vec![0, 1, 2, 3];
    let lambda: fn(&mut i32) -> i32 = |x| *x + 1;
    // vec.iter_mut().for_each(lambda);
    for x in &mut vec {
        *x = lambda(&mut *x);
    }
    println!("{:?}", vec);
}

#[test]
fn test_mutate_void() {
    let mut vec = vec![0, 1, 2, 3];
    let lambda: fn(&mut i32) -> () = |x| *x += 1;
    // vec.iter_mut().for_each(lambda);
    for x in &mut vec {
        lambda(&mut *x);
    }
    println!("{:?}", vec);
}