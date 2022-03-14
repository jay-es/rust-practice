#![allow(dead_code)]

// 新しい型に既存のトレイトを実装する

use std::cmp::Ordering;
use std::cmp::PartialEq;
use std::ops::Add;

#[derive(Debug, Clone, Copy)]
// 二次元空間上の点を表す構造体
struct Point2d {
    x: f64,
    y: f64,
}

impl Point2d {
    // 原点からの距離の二乗
    fn distance_sq(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2)
    }
}

// 加算できるようにするために、Add トレイトを実装
impl Add for Point2d {
    type Output = Self;

    fn add(self, rhs: Self) -> <Self as Add>::Output {
        // 成分ごとに加えて、新しいインスタンスを返す
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// 大小比較のために PartialEq と PartialOrd トレイトを実装
impl PartialEq for Point2d {
    fn eq(&self, other: &Self) -> bool {
        // distance_sq の大小を、Point2d の大小とする
        let dist_self_sq = self.distance_sq();
        let dist_other_sq = other.distance_sq();
        dist_self_sq.eq(&dist_other_sq)
    }
}

impl PartialOrd for Point2d {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let dist_self_sq = self.distance_sq();
        let dist_other_sq = other.distance_sq();

        // f64 に帰着すれば、既存の f64 に対する partial_cmp が使える
        dist_self_sq.partial_cmp(&dist_other_sq)
    }
}

pub fn fn1() {
    let x = Point2d { x: 3.0, y: 4.0 };
    let y = Point2d { x: 6.0, y: 8.0 };
    let z = Point2d { x: 4.0, y: 3.0 };

    // Point2d に対しても + が使える
    println!("x + y  : {:?}", x + y);

    // Point2d に対しても <, =, > で大小比較ができる
    println!("x > y? : {}", x > y);

    // 成分が異なっても原点からの距離が等しければ、等しいとみなされる
    println!("x == z? : {}", x == z);
}
