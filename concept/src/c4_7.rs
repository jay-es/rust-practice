#![allow(dead_code)]

// 列挙型

use std::cmp::Ordering;

enum Sign {
    Positive,
    Zero,
    Negative,
}

fn determine_sign(x: i32) -> Sign {
    match x.cmp(&0) {
        Ordering::Greater => Sign::Positive,
        Ordering::Equal => Sign::Zero,
        Ordering::Less => Sign::Negative,
    }
}

fn print_sign(s: Sign) {
    match s {
        Sign::Positive => println!("+"),
        Sign::Zero => println!("0"),
        Sign::Negative => println!("-"),
    }
}

pub fn fn1() {
    print_sign(determine_sign(1));
    print_sign(determine_sign(-2));
    print_sign(determine_sign(0));
}

// データあり
enum EnumExample {
    TupleTypeExample1(String),
    TupleTypeExample2(i32, bool),
    StructTypeExample { name: String, age: u8 },
}

pub fn fn2() {
    let v: Vec<EnumExample> = vec![
        (EnumExample::TupleTypeExample1(String::from("Hello"))),
        (EnumExample::TupleTypeExample2(10, true)),
        (EnumExample::StructTypeExample {
            name: String::from("taro"),
            age: 10,
        }),
    ];

    // 特定のパターンを取り出すのは if let
    for e in &v {
        if let EnumExample::StructTypeExample { name: n, age: a } = e {
            println!("StructTypeExample if_let: name = {}, age = {}", n, a);
        }
    }

    // 全パターンを書く場合は match
    for e in &v {
        match e {
            EnumExample::TupleTypeExample1(s) => {
                println!("TupleTypeExample1: s = {}", s)
            }
            EnumExample::TupleTypeExample2(n, b) => {
                println!("TupleTypeExample1: n = {}, b = {}", n, b)
            }
            EnumExample::StructTypeExample { name: n, .. } => {
                println!("StructTypeExample: name = {}", n);
            }
        }
    }
}
