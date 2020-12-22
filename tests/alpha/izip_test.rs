macro_rules! lambda {
    // @closure creates a tuple-flattening closure for .map() call. usage:
    // @closure partial_pattern => partial_tuple, rest, of, iterators
    // eg. lambda!( @closure ((a, b), c) => (a, b, c), dd, ee )
    ( @:: $args:pat => $body:expr ) => { |$args| $body };
    // // The "b" identifier is a different identifier on each recursion level thanks to hygiene.
    // ( @:: $p:pat => ( $($tup:tt)* ), $_iter:expr $(, $tail:expr )* ) => {
    //     lambda!(@:: ($p, b) => ( $($tup)*, b ) $(, $tail )*)
    // };
}

macro_rules! flat_tuple {
    ( @:: $tuple:pat => ( $($heads:tt)* ), $tail:expr ) => {
        lambda!(@:: ($tuple, x) => ( $($heads)*, x ))
    };
    ( @:: $tuple:pat => ( $($heads:tt)* ), $element:expr $(, $tail:expr )* ) => {
        flat_tuple!(@:: ($tuple, x) => ( $($heads)*, x ) $(, $tail)* )
    };
}

macro_rules! zipper {
    // binary
    ($first:expr, $second:expr $(,)*) => {
        $first.into_iter().zip($second)
    };
    // n-ary where n > 2
    ($first:expr $(, $rest:expr)* $(,)*) => {
        $first.into_iter()$(.zip($rest))*
            .map(flat_tuple!(@:: body => (body) $(,$rest)*))
    };
}

#[test]
fn zipper_test() {
    // iterate over three sequences side-by-side
    let mut container = [0, 0, 0, 0];
    let inputs = [3, 7, 9, 6];
    let result = zipper!(&container, 0..10, &inputs)
        .map(|(a, b, c)| a + b as i32 + c).collect::<Vec<i32>>();
    println!("{:?}", result);
    for (element, index, input) in zipper!(&mut container, 0..10, &inputs) {
        *element = index * 10 + input;
    }

    assert_eq!(container, [0 + 3, 10 + 7, 29, 36]);
    println!("{:?}", container);
}