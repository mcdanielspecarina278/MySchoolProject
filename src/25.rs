// 本示例演示如何使用 Rust 进行文件操作。该示例创建了一个名为 "example.txt" 的文件，并将其内容写入到当前目录中。
use std::fs::{self, File};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // 1. 创建一个名为 "example.txt" 的文件
    let mut file = File::create("example.txt")?;
    writeln!(file, "This is a test.")?;

    // 2. 写入内容到文件
    write!(&mut file, "Hello, Rust!")?;
    println!("File content written.");

    Ok(())
}
