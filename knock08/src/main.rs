use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // ファイルの内容を1行ずつ出力する
    let file = match File::open("./knock08/src/data.txt") {
        Ok(file) => file,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(e) => println!("Error: {}", e),
        }
    }
}
