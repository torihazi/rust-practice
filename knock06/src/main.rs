use std::io;
fn main() {
    // 文字列の入力を受け取る
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: &str = input.trim();
    // 文字列を反転する
    let reversed = input.chars().rev().collect::<String>();
    println!("{}", reversed);
    // 文字列を大文字に変換する
    let upper = reversed.to_uppercase();
    println!("{}", upper);
    // 文字列を小文字に変換する
    let lower = upper.to_lowercase();
    println!("{}", lower);
}
