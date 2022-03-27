#![allow(dead_code)]

/** イテレータ */

// レンジ

pub fn fn1() {
    let mut r = 1..3; // 1, 2

    println!("{:?}", r.next()); // Some(1)
    println!("{:?}", r.next()); // Some(2)
    println!("{:?}", r.next()); // None
}

// Vec<T> 型に対するイテレータ

// ケース1 Vec<T,A>型の値
pub fn fn2_1() {
    let vv = vec![1, 2, 3, 4];
    let mut iter = vv.into_iter();

    let x = iter.next().unwrap();
    println!("{}", x);

    let x = iter.next().unwrap();
    println!("{}", x);

    // into_iter によって所有権が移動しているので、以下はエラーになる
    // let x = vv[2];
    // println!("{}", x);
}

// ケース2 イミュータブルなリファレンス
pub fn fn2_2() {
    let vv = vec![1, 2, 3, 4];
    let mut iter = (&vv).into_iter();

    let x = iter.next().unwrap();
    println!("{}", *x);

    let x = iter.next().unwrap();
    println!("{}", *x);

    // 所有権は移動していないので vv にアクセスできる
    let x = vv[2];
    println!("{}", x);
}

// ケース ミュータブルなリファレンス
pub fn fn2_3() {
    let mut vv = vec![1, 2, 3, 4];
    let mut iter = (&mut vv).into_iter();

    let x = iter.next().unwrap();
    println!("{}", *x);

    let x = iter.next().unwrap();
    println!("{}", *x);

    // 参照先の値を更新すると vv の値も変わる
    *x += 10;
    println!("{:?}", vv);
}

// Rust での for ループ
pub fn fn3_1() {
    let vv = vec![1, 2, 3, 4];

    // vv の所有権がイテレータに移動する
    for ii in vv {
        print!("{}", ii);
    }
    println!();

    // 所有権が移動しているのでエラー
    // for ii in vv {
    //     println!("{}", ii);
    // }
    // println!();
}
pub fn fn3_2() {
    let vv = vec![1, 2, 3, 4];

    for ii in &vv {
        print!("{}", ii);
    }
    println!();

    // リファレンスならOK
    for ii in &vv {
        print!("{}", ii);
    }
    println!();
}

// collect()
pub fn fn4() {
    let c = "あいうえお";
    let c_vec = c.chars().collect::<Vec<char>>();

    println!("{:?}", c_vec); // ['あ', 'い', 'う', 'え', 'お']

    // char は自明なので省略可
    let _c_vec = c.chars().collect::<Vec<_>>();

    // 型アノテーションでもOK
    let _c_vec: Vec<_> = c.chars().collect();
}
