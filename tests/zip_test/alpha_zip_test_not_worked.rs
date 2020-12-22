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
    ( @:: $tuple:pat => ( $($head:tt)* ), $tail:expr ) => {
        lambda!(@:: ($tuple, x) => ( $($head)*, x ))
    };
    ( @:: $tuple:pat => ( $($head:tt)* ), $element:expr $(, $tail:expr )* ) => {
        flat_tuple!(@:: ($tuple, x) => ( $($head)*, x ) $(, $tail)* )
    };
}


macro_rules! zipper {
    // binary
    ($first:expr, $second:expr $(,)*) => {
        $first.into_iter().zip($second)
    };
    // n-ary where n > 2
    ($first:expr $(, $rest:expr)* $(,)*) => {
        // println!("a");
        $first.into_iter()$(.zip($rest))*
            .map(flat_tuple!(@:: body => (body) $(,$rest)*))
    };
    // ($first:expr, $second:expr $(,)*; $func:expr; $type:ty) => {
    //     $first.into_iter().zip($second).map($func).collect::<Vec<$type>>();
    // };
    ($first:expr, $second:expr $(,)*; $func:expr; $type:ty) => {
        $first.into_iter().zip($second).map(|(x, y)| $func(x,y)).collect::<Vec<$type>>();
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn tri_zipper_test() {
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

    #[test]
    fn qua_zipper_test() {
        // iterate over three sequences side-by-side
        let a = [10, 12, 14, 16];
        let b = [20, 22, 24, 26];
        let c = [30, 32, 34, 36];
        let d = [40, 42, 44, 46];
        let v = zipper!(&a, &b, &c, &d)
            .map(|(a, b, c, d)| (a + b + c) + d)
            .collect::<Vec<i32>>();
        println!("v = {:?}", v);
    }

    #[test]
    fn dual_zipper_test() {
        let a = [1, 2, 3, 4];
        let b = [2, 4, 8, 16];
        let v = zipper!(&a, &b; | a, b | a + b; i32);
        println!("{:?}", v);
    }
}