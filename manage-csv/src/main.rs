// 1. 必要なクレートをインポート
use std::fs::File;
use csv::ReaderBuilder;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let count = count_lines_from_position("./manage-csv/data.csv", 3)?;
    println!("{}", count);
    Ok(())
}

fn count_lines_from_position(file_path: &str, start_position: usize) -> Result<usize, Box<dyn std::error::Error>> {

    let file = File::open(file_path)?;
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);

    let mut count = 0;
    let mut current_position = 0;

    for result in reader.records() {
        current_position += 1;
        println!("current_position: {}", current_position);
        println!("result: {:?}", result);
        if current_position >= start_position {
            count += 1;
            println!("count: {}", count);
        }
    }

    Ok(count)
}