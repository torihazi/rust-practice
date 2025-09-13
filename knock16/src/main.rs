use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ファイルの内容を読み込み、対象の文字列があるか検索する。
    // ファイルの内容を読み込み、文字列として取得する。
    let file_str = fs::read_to_string("./knock16/src/data.txt")?;

    // コマンドライン引数として対象の文字列を取得する
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        return Err("target is empty".into());
    }
    let target = &args[1];
    // file_strをiteratorとして1行ずつ調べ、targetがある箇所を行数とともに出力する。
    for (index, line) in file_str.lines().enumerate() {
        if line.contains(target) {
            println!("lines{}: {}", index + 1, line);
        }
    }

    Ok(())
}
