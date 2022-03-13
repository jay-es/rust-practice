#![allow(dead_code)]

// Option<T>

// if で判定して Option 型を生成
fn div1(x: i32, y: i32) -> Option<i32> {
    if y == 0 {
        None
    } else {
        Some(x / y)
    }
}

// match で判定して Option 型を生成
fn div2(x: i32, y: i32) -> Option<i32> {
    match y {
        0 => None,
        _ => Some(x / y),
    }
}

// if let で処理
fn print_some1<T: std::fmt::Display>(ans: Option<T>) {
    if let Some(x) = ans {
        println!("{}", x)
    } else {
        println!("None")
    }
}

// match で処理
fn print_some2<T: std::fmt::Display>(ans: Option<T>) {
    match ans {
        Some(x) => println!("{}", x),
        None => println!("None"),
    }
}

pub fn fn1() {
    let ans1 = div1(10, 2);
    let ans2 = div2(10, 0);

    print_some1(ans1);
    print_some2(ans2);

    // メソッド

    println!("ans1.unwrap: {}", ans1.unwrap());
    // ans2.unwrap(); // None なので panic
    // ans2.expect("None なので panic"); // panic のエラーメッセージを指定
    println!("ans2.unwrap_or: {}", ans2.unwrap_or(10)); // -> 10
    println!("ans2.unwrap_or_default: {}", ans2.unwrap_or_default()); // -> 0

    println!("ans1.is_some: {}", ans1.is_some());
    println!("ans2.is_none: {}", ans2.is_none());

    // プリミティブでない値

    let st = Some(String::from("Hello"));
    print_some1(st);
    // print_some2(st); // 上の行で所有権が移ったのでエラーになる
}
