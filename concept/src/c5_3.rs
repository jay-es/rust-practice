#![allow(dead_code)]

// ジェネリクスの型に対するトレイトによる制限

// トレイトの定義（他言語の interface に似ている）
trait CalcArea {
    fn calc_area(&self) -> f64;
}

// 長方形
struct Rectangle {
    width: f64,  // 横
    height: f64, // 縦
}

// Rectangle に CalcArea トレイトを実装
impl CalcArea for Rectangle {
    fn calc_area(&self) -> f64 {
        self.width * self.height
    }
}

// 直角三角形
struct RightTriangle {
    width: f64,  // 底辺
    height: f64, // 高さ
}

impl CalcArea for RightTriangle {
    fn calc_area(&self) -> f64 {
        self.width * self.height / 2.0
    }
}

// 型パラメータの成約（トレイト境界）
fn area<T: CalcArea>(x: &T) -> f64 {
    x.calc_area()
}

// where キーワードで書くこともできる
fn area2<T>(x: &T) -> f64
where
    T: CalcArea,
{
    x.calc_area()
}

pub fn fn1() {
    let rect = Rectangle {
        width: 1.0,
        height: 2.0,
    };
    let tri = RightTriangle {
        width: 1.0,
        height: 2.0,
    };

    println!("rect area = {}", area(&rect));
    println!("tri area = {}", area(&tri));
}

trait CalcLength {
    fn calc_length(&self) -> f64;
}

// 線分
struct Line {
    length: f64,
}

impl CalcLength for Line {
    fn calc_length(&self) -> f64 {
        self.length
    }
}

impl CalcLength for Rectangle {
    fn calc_length(&self) -> f64 {
        (self.width + self.height) * 2.0
    }
}

impl CalcLength for RightTriangle {
    fn calc_length(&self) -> f64 {
        self.width + self.height + (self.width.powi(2) + self.height.powi(2)).sqrt()
    }
}

fn length<T: CalcLength>(x: &T) -> f64 {
    x.calc_length()
}

pub fn fn2() {
    let rect = Rectangle {
        width: 1.0,
        height: 2.0,
    };
    println!("rect area = {}", area(&rect));
    println!("rect length = {}", length(&rect));

    let tri = RightTriangle {
        width: 1.0,
        height: 2.0,
    };
    println!("tri area = {}", area(&tri));
    println!("tri length = {}", length(&tri));

    let line = Line { length: 5.0 };
    // println!("line area = {}", area(&line));
    println!("line length = {}", length(&line));
}

// メソッドのデフォルト実装

trait PrintHello {
    fn print_hello(&self) {
        println!("Hello!");
    }
}

struct Test1;
struct Test2;

// print_hello デフォルトの内容で実装
impl PrintHello for Test1 {}

// デフォルト実装を上書き
impl PrintHello for Test2 {
    fn print_hello(&self) {
        println!("Hello, world")
    }
}

pub fn fn3() {
    let test1 = Test1;
    test1.print_hello();

    let test2 = Test2;
    test2.print_hello();
}
