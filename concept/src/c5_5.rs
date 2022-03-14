#![allow(dead_code)]

// トレイトの関連型

trait IAbs {
    type Output;
    fn iabs(self) -> <Self as IAbs>::Output;
}

impl IAbs for i32 {
    type Output = u32;
    fn iabs(self) -> <Self as IAbs>::Output {
        if self >= 0 {
            self as <Self as IAbs>::Output
        } else {
            -self as u32 // ← ハードコードしても動くがおそらく非推奨
        }
    }
}

pub fn fn1() {
    let abs1 = 1.iabs();
    let abs2 = (-1).iabs();
    println!("{}", abs1);
    println!("{}", abs2);
}

// トレイトのデフォルト実装の活用

use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt::Debug;
use std::ops::Neg;

trait IAbs2 {
    type Output;

    // すべての型で使えるようなデフォルト実装
    fn iabs2(self) -> <Self as IAbs2>::Output
    where
        Self: Sized // スタック領域に配置するため、サイズが決まっていることを表すトレイト
            + PartialOrd // 0 との大小比較
            + Neg // -1 倍する
            + From<i8> // i8 から変換できる
            + TryInto<<Self as IAbs2>::Output>, // Output 型に変換できる
        // <Self as IAbs2>::Output は -1 倍した型 `<Self as Neg>::Output` から作れる必要がある
        <Self as IAbs2>::Output: TryFrom<<Self as Neg>::Output>,
        // try_into() の返却値は Result で、unwrap() が動作するためには Error に Debug トレイトが必要
        // 正と負で Result のエラー型が違うので、それぞれに Debug トレイトを実装
        <Self as TryInto<<Self as IAbs2>::Output>>::Error: Debug,
        <<Self as IAbs2>::Output as TryFrom<<Self as Neg>::Output>>::Error: Debug,
    {
        if self >= (0_i8).into() {
            self.try_into().unwrap()
        } else {
            (-self).try_into().unwrap()
        }
    }
}

impl IAbs2 for i32 {
    type Output = u32;
}

impl IAbs2 for i8 {
    type Output = u8;
}

impl IAbs2 for i16 {
    type Output = u16;
}

impl IAbs2 for i64 {
    type Output = u64;
}

pub fn fn2() {
    println!("{:?}", (-2_i32).iabs2());
    println!("{:?}", (5_i8).iabs2());
    println!("{:?}", (-6_i16).iabs2());
}
