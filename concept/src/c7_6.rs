#![allow(dead_code)]

/** パターンマッチの活用 */

fn insert(x: i32, xs: &[i32]) -> Vec<i32> {
    match xs {
        [y, ys @ ..] => {
            if x <= *y {
                [&[x][..], xs].concat()
            } else {
                [&[*y][..], &insert(x, ys)].concat()
            }
        }
        [] => vec![x],
    }
}

pub fn fn1() {
    let v: Vec<i32> = Vec::new();
    let v = insert(2, &v);
    println!("{:?}", v); // [2]

    let v = insert(1, &v);
    println!("{:?}", v); // [1, 2]

    let v = insert(3, &v);
    println!("{:?}", v); // [1, 2, 3]
}
