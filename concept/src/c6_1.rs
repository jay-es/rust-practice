#![allow(dead_code)]

// ファイルの内容を固定長のバッファに繰り返し読み出す

use std::fs::File;
use std::io::Read;

const BUFSIZE: usize = 1024;

pub fn fn1() -> std::io::Result<()> {
    let mut f = File::open("../.gitignore")?;
    let mut buf = [0_u8; BUFSIZE];

    let mut lines = Vec::new();
    let mut linebuf = String::new();

    loop {
        let read_size = f.read(&mut buf)?;

        if read_size == 0 {
            break;
        }

        for cc in &buf[..read_size] {
            if *cc == b'\n' {
                lines.push(linebuf);
                linebuf = String::new();
            } else {
                linebuf.push(*cc as char);
            }
        }
    }

    println!("{:?}", lines);

    Ok(())
}

// 一括バージョン
pub fn fn2() -> std::io::Result<()> {
    let mut f = File::open("Cargo.toml")?;

    let mut lines = Vec::new();
    let mut linebuf = String::new();

    let mut buf = Vec::new();
    let read_size = f.read_to_end(&mut buf)?;

    for cc in &buf[..read_size] {
        if *cc == b'\n' {
            lines.push(linebuf);
            linebuf = String::new();
        } else {
            linebuf.push(*cc as char);
        }
    }

    println!("{:?}", lines);

    Ok(())
}

// BufReader を使って1行ずつ読み出す

use std::io::{self, BufRead, BufReader};
// use std::fs::File; // 上でも import しているので不要

pub fn fn3() -> io::Result<()> {
    let f = File::open("../README.md")?;
    let f = BufReader::new(f);

    let mut lines_vec = Vec::new();

    for ll in f.lines() {
        lines_vec.push(ll.unwrap());
    }

    println!("{:?}", lines_vec);
    Ok(())
}

// モジュール化（本当は別ファイルに作成）
pub fn get_lines<T: Read>(f: T) -> Vec<String> {
    let f = BufReader::new(f);
    let mut lines_vec = Vec::new();

    for ll in f.lines() {
        lines_vec.push(ll.unwrap());
    }

    lines_vec
}

pub fn fn35() -> io::Result<()> {
    let f = File::open("../README.md")?;
    let lines_vec = get_lines(f);

    println!("{:?}", lines_vec);
    Ok(())
}
// ファイルの書き込み

// use std::fs::File;
// use std::io::Read;
use std::io::Write;

// const BUFSIZE: usize = 1024;

pub fn fn4() -> std::io::Result<()> {
    let mut fr = File::open("src/c6_1.rs")?;
    let mut fw = File::create("target/c6_1.rs")?;

    let mut buf = [0_u8; BUFSIZE];

    loop {
        let read_size = fr.read(&mut buf)?;

        if read_size == 0 {
            break;
        } else {
            let _ = fw.write(&buf[..read_size]);
        }
    }

    Ok(())
}

// ファイルを開く際のオプションの指定

use std::fs::OpenOptions;

fn fn5() {
    let _ = OpenOptions::new()
        .read(true) // 読み取り可否
        .write(true) // 書き込み可否
        .append(true) // 追記モード
        .open("file.txt");
}
