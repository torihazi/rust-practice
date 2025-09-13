use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        return Err("target is empty".into());
    }

    let target = &args[1];

    // 辞書ファイルを読み込む
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path("./knock17/src/dictionary.txt")?;

    // hashmapを作成
    let mut dict = std::collections::HashMap::new();

    // 辞書ファイルをもとにdictに反映
    for result in rdr.records() {
        let record = result?;
        dict.insert(record[0].to_string(), record[1].to_string());
    }

    match dict.get(target) {
        Some(value) => println!("{}", value),
        None => println!("{} is not found", target),
    }

    Ok(())
}
