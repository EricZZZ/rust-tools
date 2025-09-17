use std::collections::HashMap;
use std::{
    error::Error,
    io::{self},
    path,
};
type Record = HashMap<String, String>;
fn main() -> Result<(), Box<dyn Error>> {
    // 命令行中输入文件路径
    println!("Welcome to csv reader!");
    println!("Please input file path: ");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    // let file_path = buffer.trim();
    let file_path = buffer.trim();
    // 判断文件是否存在，并且文件是csv格式的
    if !path::Path::new(file_path).exists() {
        eprintln!("File not found: {}", file_path);
        std::process::exit(1);
    }
    if !file_path.ends_with(".csv") {
        eprintln!("File is not a csv file: {}", file_path);
        std::process::exit(1);
    }
    // 读取文件
    let mut content = csv::Reader::from_path(path::Path::new(file_path))?;
    // 打印文件内容
    for record in content.deserialize() {
        let record: Record = record?;
        println!("{:?}", record);
    }
    Ok(())
}
