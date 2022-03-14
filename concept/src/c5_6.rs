#![allow(dead_code)]

// クラスによる抽象化との比較

/*
 * class による実装の場合、Speaker クラスを継承して EnglishPerson などを作ることになる
 * Rust では「動作」をトレイトで定義して、EnglishPerson などに実装していく
 */

trait SayHello {
    fn say_hello(&self);
}

trait SayThankyou {
    fn say_thankyou(&self);
}

struct EnglishPerson;
struct SpanishPerson;

impl SayHello for EnglishPerson {
    fn say_hello(&self) {
        println!("Hello")
    }
}

impl SayThankyou for EnglishPerson {
    fn say_thankyou(&self) {
        println!("Thank you")
    }
}

impl SayHello for SpanishPerson {
    fn say_hello(&self) {
        println!("Hola")
    }
}

impl SayThankyou for SpanishPerson {
    fn say_thankyou(&self) {
        println!("Gracias")
    }
}

fn say_hello_general<T: SayHello>(speaker: &T) {
    speaker.say_hello()
}

fn say_thankyou_general<T: SayThankyou>(speaker: &T) {
    speaker.say_thankyou()
}

pub fn fn1() {
    let en = EnglishPerson;
    let sp = SpanishPerson;

    say_hello_general(&en);
    say_hello_general(&sp);

    say_thankyou_general(&en);
    say_thankyou_general(&sp);
}

// 新しいコードを追加する

/*
 * class による実装の場合、Person = Sperker + Runner クラスを作成、EnglishPerson などに継承させる
 * （既存コードに手を入れる必要がある）
 * Rust では Run をトレイト追加すればよく、既存のコードを修正することなく機能を追加できる
 */

trait Run {
    fn run(&self);
}

impl Run for EnglishPerson {
    fn run(&self) {
        println!("Run")
    }
}

impl Run for SpanishPerson {
    fn run(&self) {
        println!("Correr")
    }
}

fn say_thankyou_and_run<T: SayThankyou + Run>(person: &T) {
    person.say_thankyou();
    person.run();
}

pub fn fn2() {
    let en = EnglishPerson;
    let sp = SpanishPerson;

    say_thankyou_and_run(&en);
    say_thankyou_and_run(&sp);
}
