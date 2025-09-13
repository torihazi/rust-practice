fn main() {
    //CSVファイルを読み込み、出力
    // csv parserを定義
    let mut csv_parser = csv::Reader::from_path("./knock13/src/data.csv").unwrap();
    // csv parserを使ってデータを出力
    for result in csv_parser.records() {
        let record = match result {
            Ok(record) => record,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };

        for field in record.iter() {
            print!("{} ", field);
        }
        println!()
    }
}
