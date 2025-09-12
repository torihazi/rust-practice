use rand::Rng;
use std::io;

fn main() {
    let mut rng = rand::rng();
    loop {
        println!("サイコロを振ります。");
        let dice_number = rng.random_range(1..6);
        println!("出目: {}", dice_number);
        println!("もう一度やりますか？(y/n)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "n" {
            break;
        }
    }
}
