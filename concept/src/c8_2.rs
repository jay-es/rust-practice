#![allow(dead_code)]

/** スレッドによる並列実行 */
use std::thread::spawn;

pub fn fn1() {
    let mut v_threads = Vec::new();

    for i in 0..10 {
        let thread = spawn(move || println!("{}", i));
        v_threads.push(thread);
    }

    let _x: Vec<()> = v_threads.into_iter().map(|th| th.join().unwrap()).collect();
}

pub fn fn2() {
    let mut v_threads = Vec::new();
    let hello = String::from("Hello"); // ムーブセマンティクスが適用される

    for i in 0..10 {
        // 1 回目のループで hello の所有権がクロージャに移ってしまうのでエラー
        // let thread = spawn(move || println!("{}: {}", i, hello));

        // .clone() でコピーしてから渡す
        let hello_cloned = hello.clone();
        let thread = spawn(move || println!("{}: {}", i, hello_cloned));

        v_threads.push(thread);
    }

    let _x: Vec<()> = v_threads.into_iter().map(|th| th.join().unwrap()).collect();
}

// スレッドによる並列実行を活用したベクターの要素の和の計算
const N_MAX: usize = 40000000;
const N_THREAD: usize = 4;

const N_ELEM_PER_THRD: usize = N_MAX / N_THREAD;
const RESIDUAL: usize = N_MAX % N_THREAD;

pub fn fn3() -> std::thread::Result<()> {
    if RESIDUAL != 0 {
        panic!("invalid combination of N_MAX and N_THREAD");
    }

    println!("start");

    let mut thrd = Vec::new();
    let v = (1..=N_MAX).collect::<Vec<usize>>();

    // 1..N_MAX を N_THREAD に分割して、それぞれの和をスレッドで計算
    for ii in 0..N_THREAD {
        let ist = ii * N_ELEM_PER_THRD;
        let ien = ist + N_ELEM_PER_THRD;

        // .to_owned() によって、リファレンスが指している値をコピー
        let vv = (&v[ist..ien]).to_owned();
        let th = spawn(move || vv.into_iter().sum::<usize>());

        thrd.push(th);
    }

    // 各スレッドで計算した値を集めて、その和を取り、全体の和を求める
    let ans: usize = thrd.into_iter().map(|r| r.join().unwrap()).sum::<usize>();
    println!("{}", ans);
    assert_eq!(ans, N_MAX * (N_MAX + 1) / 2);
    Ok(())
}
