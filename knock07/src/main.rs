use std::env;

fn main() {
    // 文字列の長さを数える
    let args: Vec<String> = env::args().collect();
    println!("文字列の長さ : {}", args[1].len());
}
