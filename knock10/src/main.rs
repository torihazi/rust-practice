use std::thread;
use std::time::Duration;

fn main() {
    // カウントアップタイマー
    let mut count = 0;
    println!("カウントアップタイマー");
    loop {
        println!("{}", count);
        count += 1;
        thread::sleep(Duration::from_secs(1));
    }
}
