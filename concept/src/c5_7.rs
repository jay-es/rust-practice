#![allow(dead_code)]

// トレイトオブジェクト

/*
 * 「std::error::Error トレイトを実装している型」は以下のどちらかで型を指定する。dyn は dynamic の略
 * &dyn std::error::Error
 * Box<dyn std::error::Error>
 */

// 実行時に決まるエラーの型

trait MyError: std::fmt::Debug {}

#[derive(Debug)]
struct MyError1;
impl MyError for MyError1 {}

#[derive(Debug)]
struct MyError2;
impl MyError for MyError2 {}

#[derive(Debug)]
struct MyError3;
impl MyError for MyError3 {}

#[derive(Debug)]
struct MyErrorOther;
impl MyError for MyErrorOther {}

// Result の E を「MyError トレイトを実装している型」として指定
fn div4(x: i32) -> Result<(), Box<dyn MyError>> {
    let res = x % 4;
    match res {
        0 => Ok(()),
        1 => Err(Box::new(MyError1 {})),
        2 => Err(Box::new(MyError2 {})),
        3 => Err(Box::new(MyError3 {})),
        _ => Err(Box::new(MyErrorOther {})),
    }
}

pub fn fn1() {
    println!("{:?}", div4(0));
    println!("{:?}", div4(1));
    println!("{:?}", div4(2));
    println!("{:?}", div4(3));
}

// 異なる型を要素とする Vec<T> 型
pub fn fn2() {
    #![allow(clippy::vec_init_then_push)]
    let mut v = Vec::<Box<dyn std::fmt::Debug>>::new();

    v.push(Box::new(1_i32));
    v.push(Box::new(2_i32));
    v.push(Box::new(3.0_f64));
    v.push(Box::new(String::from("Hello")));

    println!("{:?}", v);
    println!("{:?}", v[0]); // i32 ではなく dyn std::fmt::Debug 型
}
