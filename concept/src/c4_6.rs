#![allow(dead_code)]

// 構造体

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

pub fn fn1() {
    // インスタンス作成
    let taro = Person {
        name: String::from("taro"),
        age: 10,
    };

    let jiro = Person {
        name: String::from("jiro"),
        ..taro // ピリオド2つ + カンマなし
    };

    println!("{}", taro.name);
    println!("{}", taro.age);

    // Debug トレイトを実装すると :? を使って見やすい形式に出力できる
    println!("{:?}", jiro);
}

impl Person {
    // 関連関数
    fn new(name: String, age: u8) -> Person {
        Person { name, age }
    }

    // メソッド
    fn age_incr(&self, incr: u8) -> u8 {
        self.age + incr
    }

    fn age_incr_replace(&mut self, incr: u8) {
        self.age += incr
    }
}

pub fn fn2() {
    let mut taro = Person::new(String::from("taro"), 10);

    let age_plus1 = taro.age_incr(1);
    println!("{}", age_plus1);

    taro.age_incr_replace(10);
    println!("{}", taro.age)
}
