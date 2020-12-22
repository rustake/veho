#[macro_export] // #[macro_export] 注解说明宏可用
macro_rules! vector { // macro_rules!后开始定义宏，宏名不带感叹号
    ( $( $x:expr ),* ) => {
    // (...) 整个模式
    // $(...) 捕获符合括号内模式的值, 用于替换后的代码
    // $x:expr expr: 匹配表达式, $x: 将匹配的表达式记作$x
    // * 此模式匹配零至多个
        {
            let mut vec = Vec::new();
            $(vec.push($x);)*
            vec
        }
    };
}

#[test]
fn declarative_marco_test() {
    let vec = vector![1, 2, 3];
    println!("{:?}", vec);
}