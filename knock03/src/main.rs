use rand::Rng;
use std::io;

fn main() {
    // 乱数生成期の初期化
    let mut rng = rand::rng();

    loop {
        let random_number = rng.random_range(1..100);
        if random_number % 2 == 0 {
            println!("表");
        } else {
            println!("裏");
        }

        println!("もう一度やりますか？(y/n)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        // trimしないと改行が入って "n/n"となり、falseになる
        if input.trim() == "n" {
            break;
        }
    }
}
