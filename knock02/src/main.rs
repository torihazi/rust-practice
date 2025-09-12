use std::io;

fn main() {
    let mut sum: i32 = 0;
    loop {
        println!("Enter a number: ");
        // 数字の入力を受け取る
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input: &str = input.trim();

        if input == "q" {
            break;
        }
        match input.parse::<i32>() {
            Ok(num) => sum += num,
            Err(_) => println!("Invalid input"),
        }
    }
    println!("Sum: {}", sum);
}
