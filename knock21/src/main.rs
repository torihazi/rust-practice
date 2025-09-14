use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 現在の実行ディレクトリを取得
    let current_dir = env::current_dir()?;
    let target_dir = current_dir.join("knock21");

    //　複数ファイルのサイズを調べる
    let file_names = vec!["file1.txt", "file2.txt", "file3.txt", "file4.txt"];

    // 合計サイズを求める
    let mut total_size = 0;
    for file_name in file_names {
        let metadata = match fs::metadata(generate_target_path(&target_dir, file_name)) {
            Ok(metadata) => metadata,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                println!("{}: file does not exist", file_name);
                continue;
            }
            Err(err) => return Err(err.into()),
        };

        let size = metadata.len();
        println!("{}: {}", file_name, size);
        total_size += size;
    }

    println!("Total size: {}", total_size);

    Ok(())
}

fn generate_target_path(target_dir: &Path, file_name: &str) -> PathBuf {
    target_dir.join(file_name)
}
