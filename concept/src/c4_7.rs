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

pub fn main() {
    print_sign(determine_sign(1));
    print_sign(determine_sign(-2));
    print_sign(determine_sign(0));
}
