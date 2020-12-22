#[macro_export] // #[macro_export] 注解说明宏可用
macro_rules! vector2 { // macro_rules!后开始定义宏，宏名不带感叹号
    ( $closure:tt $(, $element:expr )*  ) => {
        {
            println!("{}", $closure);
            let mut vec = Vec::new();
            $(vec.push($element);)*
            vec
        }
    };
}

#[test]
fn declarative_marco_test() {
    let vec = vector2![0,1, 2, 3];
    println!("{:?}", vec);
}