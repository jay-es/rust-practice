#![allow(dead_code)]

/** 非同期処理による並列実行 */

// Rust における非同期処理

async fn sum_func(n: usize) -> usize {
    let ans = (1..=n).into_iter().sum::<usize>();
    println!("{}", ans);
    ans
}

pub fn fn1() {
    let _x = sum_func(1_000_000);
    // .await するか poll を実装する必要がある（？）

    println!("called");
}

// 非同期タスクのランタイムによる実行

pub fn fn2() {
    let fut = sum_func(1_000_000);

    let ls = tokio::task::LocalSet::new();
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    ls.block_on(&rt, fut);

    // 複数の非同期関数を逐次実行するには async ブロックを使用
    ls.block_on(&rt, async {
        sum_func(1_000_000).await;
        sum_func(2_000_000).await;
    });
}

// 非同期関数の同時実行
async fn print_number(n: usize) -> usize {
    for ii in 0..2 {
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        println!("{} from {}", ii, n);
    }
    n
}

#[tokio::main] // ← これをつけると ls, rt が不要（block_on の中身だけ書けばよくなる）
pub async fn fn3() {
    let mut handler_vec = Vec::new();
    (1..=3).for_each(|ii| {
        handler_vec.push(tokio::spawn(print_number(ii)));
    });

    let ret = futures::future::join_all(handler_vec).await;
    let retval = ret.into_iter().map(|r| r.unwrap()).collect::<Vec<usize>>();
    println!("{:?}", retval);
}

// 非同期関数を用いたエコーサーバ

use std::error::Error;
use std::io;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

async fn echo_process(stream: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buf = Vec::with_capacity(1024);
    let mut input_prompt = true;

    loop {
        if input_prompt {
            stream.write("input => ".as_bytes()).await?;
        }

        let read = stream.try_read_buf(&mut buf);
        match read {
            Ok(0) => break,
            Ok(n) => {
                stream.write("output => ".as_bytes()).await?;
                stream.write_all(&buf[..n]).await?;
                input_prompt = true;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                input_prompt = false;
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
        buf.clear();
    }

    Ok(())
}

#[tokio::main]
pub async fn fn4() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("port is not specified");
    }

    let port: usize = args[1].parse().expect("Failed to get the port number");
    let addr = format!("localhost:{}", port);
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening to the port {}", port);

    loop {
        let (mut stream, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            echo_process(&mut stream).await.unwrap();
        });
    }
}
