// 1. 必要なクレートをインポート
use std::fs::File;
use csv::Reader;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 2. CSVファイルを開く
    // - File::open()でファイルハンドルを取得
    // - ファイルパスは"data.csv"を指定
    let file = File::open("data.csv")?;
    let mut reader = Reader::from_reader(file);
    
    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
        for field in record.iter() {
            print!("{} ", field);
        }
        println!();
    }
    Ok(())
}