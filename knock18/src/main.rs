use rand::{self, Rng};

fn main() {
    let charset = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
  abcdefghijklmnopqrstuvwxyz\
  0123456789";

    // 乱数生成期作成
    let mut rng: rand::prelude::ThreadRng = rand::rng();

    // 乱数を取得
    let random_str: String = (0..12)
        .map(|_| {
            // rngからcharsetの要素数の範囲で乱数を生成
            let idx = rng.random_range(0..charset.len());
            // charsetは配列なので、idxを使ってcharsetの要素を取得し、バイトからchar型に変換
            // 1文字分の型をchar, 複数文字分の型をStringに変換
            charset[idx] as char
        })
        .collect::<String>(); // collectはiteratorをまとめるが、何にまとめるかは型注釈として与える必要あり。

    println!("{}", random_str);
}
