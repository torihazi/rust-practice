use serde_json;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // JSONファイルを読み込む
    // parseする
    // 整形して出力する
    let json_str = fs::read_to_string("./knock14/src/data.json")?;
    let json: Vec<serde_json::Value> = serde_json::from_str(&json_str)?;
    println!("{:?}", json);
    println!();
    println!("{}", serde_json::to_string_pretty(&json)?);
    println!();
    Ok(())
}
