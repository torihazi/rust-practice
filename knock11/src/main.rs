use std::fs::File;
use std::io;
use std::io::Write;

fn main() {
    // メモアプリ(標準入力を受け取ってファイルにリダイレクト)
    // ファイル名を入力
    println!("ファイル名を入力: ");
    let mut filename = String::new();
    io::stdin().read_line(&mut filename).unwrap();
    let filename = filename.trim();

    // 実行ファイルの絶対パスを取得
    let exe_path = match std::env::current_exe() {
        Ok(path) => path,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    // 実行ファイルのディレクトリを取得
    let exe_dir = match exe_path.parent() {
        Some(dir) => dir.to_path_buf(),
        None => {
            println!("Error: 実行ファイルのディレクトリを取得できません");
            return;
        }
    };

    // ファイル名を絶対pathに変換
    let memos_path = exe_dir.join(filename.to_owned() + ".txt");

    // ファイルを作成
    let mut file = File::create(memos_path).unwrap();
    // 標準入力を受け取ってファイルに書き込む
    let mut input = String::new();
    println!("メモを入力: ");
    io::stdin().read_line(&mut input).unwrap();
    file.write_all(input.as_bytes()).unwrap();
}
