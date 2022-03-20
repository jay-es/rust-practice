#![allow(dead_code)]

// 抽象返却値型

fn generic_return_type<T: std::fmt::Display>(x: T) -> T {
    x
}

pub fn fn1() {
    println!("{}", generic_return_type(1));
    println!("{}", generic_return_type("Hello"));
}

// これは動かない（T ではなく integer を返している、というエラー）
// fn generic_return_type<T: std::fmt::Display>() -> T {
//     1
// }

// 返却値をトレイトオブジェクトにした場合は OK
fn generic_return_type2() -> Box<dyn std::fmt::Display> {
    Box::new(1)
}

pub fn fn2() {
    println!("{}", generic_return_type2());
}

// impl Trait による抽象返却値型

// 返却値に「std::fmt::Display トレイトを実装している匿名型」を指定
fn generic_return_type3() -> impl std::fmt::Display {
    10
}

pub fn fn3() {
    println!("{}", generic_return_type3());
}
