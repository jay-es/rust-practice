#![allow(dead_code)]

// ソケットの入出力

use std::io::{self};
use std::net::TcpListener;

#[path = "./c6_1.rs"]
mod c6_1;

pub fn fn1() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3210")?;

    for stream in listener.incoming() {
        let lines_vec = c6_1::get_lines(stream?);
        println!("{:?}", lines_vec);
    }

    Ok(())
}
