#![allow(dead_code)]

/** 関数を返却する関数 */

fn func_of_func(b: i32) -> impl Fn(i32) -> i32 {
    // b のスコープは func_of_func 内で終わる
    // move をつけると環境の値の所有権をクロージャに移動させられる
    move |x| x + b
}

pub fn fn1() {
    let add_2 = func_of_func(2);
    println!("{}", add_2(1)); // 3
    println!("{}", add_2(4)); // 6
}

// 所有権が最初の呼び出しで消費されてしまうクロージャ（FnOnce の利用）

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

fn print_point(p: Point) {
    println!("{:?}", p);
}

// 最初の実行で p の所有権が print_point に移動してしまい、二回目以降は実行できない
// fn func_of_func2(b: i32, p: Point) -> impl Fn(i32) -> i32 {
//     move |x| {
//         print_point(p);
//         x + b
//     }
// }

// 「一度しか使えない」を明示するため、返却値を FnOnce 型にする
fn func_of_func2(b: i32, p: Point) -> impl FnOnce(i32) -> i32 {
    move |x| {
        print_point(p);
        x + b
    }
}

pub fn fn2() {
    let p = Point { x: 1.0, y: 2.0 };
    let add_2 = func_of_func2(2, p);
    println!("{}", add_2(1));
}

// クロージャに閉じ込められた環境の変数を変更する場合（FnMut の利用）

// クロージャ内の値がミュータブルであることを明示するため、返却値を FnMut 型にする
fn func_of_func3(b: i32) -> impl FnMut(i32) -> i32 {
    let mut count = 0;
    move |x| {
        count += 1;
        println!("count: {}", count);
        x + b
    }
}

pub fn fn3() {
    let mut add_2 = func_of_func3(2);
    println!("{}", add_2(1)); // 3
    println!("{}", add_2(2)); // 4

    println!();

    let mut add_3 = func_of_func3(3);
    println!("{}", add_3(1)); // 4
    println!("{}", add_3(2)); // 5
}
