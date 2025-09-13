use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //ファイルを読み込み、内容の行数を出力する

    let file_str = fs::read_to_string("./knock15/src/data.txt")?;
    let lines = file_str.lines().count();
    println!("{}", lines);

    Ok(())
}
