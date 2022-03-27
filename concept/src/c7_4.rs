#![allow(dead_code)]

/** 関数を引数とする関数 */

// map()

fn add_one(x: i32) -> i32 {
    x + 1
}
pub fn fn1() {
    let v: Vec<i32> = vec![0, 1, 2, 3].into_iter().map(add_one).collect();
    println!("{:?}", v);

    // クロージャ
    let _v: Vec<i32> = vec![0, 1, 2, 3].into_iter().map(|x| x + 1).collect();

    // クロージャを変数に格納
    let add_one = |x| x + 1;
    let _v: Vec<i32> = vec![0, 1, 2, 3].into_iter().map(add_one).collect();
}

pub fn fn2() {
    let mut m = 1;
    let add_m = |x: i32| x + m;
    println!("{}", add_m(1)); // 2

    // クロージャが所有権を借用しているのでエラー
    // m = 10;
    // println!("{}", add_m(1));

    // 再宣言すればOK
    let m = 10;
    let add_m = |x: i32| x + m;
    println!("{}", add_m(1)); // 11

    // m を再宣言しても add_m には影響しない
    let m = 100;
    println!("{}", add_m(1)); // 11
}

// filter(), fold(), reduce()
pub fn fn3() {
    let v = vec![0, 1, 2, 3];
    let filtered: Vec<i32> = v.into_iter().filter(|&x| x >= 2).collect();
    println!("{:?}", filtered);

    let w = vec![4, 5, 6];
    let sum = w.into_iter().fold(0, |x, y| x + y);
    println!("{}", sum);

    // 初期値が不要な場合は reduce
    let w = vec![4, 5, 6];
    let sum = w.into_iter().reduce(|x, y| x + y).unwrap();
    println!("{}", sum);
}
