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
const N_MAX: usize = 10000000;
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

// スレッド間での共有データを参照するには

pub fn fn4() -> std::thread::Result<()> {
    if RESIDUAL != 0 {
        panic!("invalid combination of N_MAX and N_THREAD");
    }

    println!("start");

    let mut thrd = Vec::new();
    // Arc(Atomically Reference Counted) に書き換え
    let v = std::sync::Arc::new((1..=N_MAX).collect::<Vec<usize>>());

    // 1..N_MAX を N_THREAD に分割して、それぞれの和をスレッドで計算
    for ii in 0..N_THREAD {
        let ist = ii * N_ELEM_PER_THRD;
        let ien = ist + N_ELEM_PER_THRD;

        // 別の Arc 型を作り、vv に束縛
        let vv = std::sync::Arc::clone(&v);
        let th = spawn(move || vv[ist..ien].iter().sum::<usize>());

        thrd.push(th);
    }

    // 各スレッドで計算した値を集めて、その和を取り、全体の和を求める
    let ans: usize = thrd.into_iter().map(|r| r.join().unwrap()).sum::<usize>();
    println!("{}", ans);
    assert_eq!(ans, N_MAX * (N_MAX + 1) / 2);
    Ok(())
}

// スレッド間での共有データに書き込みをするには

use std::sync::{Arc, Mutex};
// use std::thread::spawn;

pub fn fn5() {
    // Mutex で保護したものを Arc データにする
    let data = Arc::new(Mutex::new(Vec::new()));
    let added = vec![1, 2, 3, 4, 5];

    let mut thrd = Vec::new();
    for aa in added {
        let data = Arc::clone(&data);
        let th = spawn(move || {
            // ロック取得
            let mut data = data.lock().unwrap();
            data.push(aa);
        });
        thrd.push(th);
    }

    thrd.into_iter().for_each(|th| {
        let _ = th.join();
    });

    // ロック取得
    let x = data.lock().unwrap();
    println!("{:?}", *x);
}

// メインスレッドとの通信

use std::sync::mpsc::channel;
// use std::thread::spawn;

pub fn fn6() {
    let data = vec![1, 2, 3, 4, 5];
    let (tx, rx) = channel();

    let data_len = data.len();

    for dd in data {
        // スレッドごとにtx をコピーして、.send() でデータを送信する
        let tx = tx.clone();
        spawn(move || tx.send(dd));
    }

    for _ in 0..data_len {
        println!("{}", rx.recv().unwrap());
    }
}

// ソケットの入出力を使ったエコーサーバ

use std::error::Error;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn echo_process(stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    #![allow(clippy::unused_io_amount)]

    let mut buf = [0_u8; 1024];
    loop {
        stream.write("input => ".as_bytes())?;
        let read = stream.read(&mut buf);

        match read {
            Ok(0) => break,
            Ok(n) => {
                stream.write("output => ".as_bytes())?;
                stream.write_all(&buf[..n])?;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    Ok(())
}

pub fn fn7() {
    let args: Vec<String> = std::env::args().collect();
    // args[0] がコマンドそのもの、args[1] が第一引数
    if args.len() != 2 {
        panic!("port is not specified");
    }

    let port: usize = args[1].parse().expect("Failed to get the port number");
    let addr = format!("localhost:{}", port);
    let listener = TcpListener::bind(addr).unwrap();
    println!("Listening to the port {}", port);

    loop {
        let (mut stream, _) = listener.accept().unwrap();
        std::thread::spawn(move || {
            echo_process(&mut stream).unwrap();
        });
    }
}
