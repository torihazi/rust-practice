use std::io;

fn main() {
    // todoリストの作成
    let mut todo_list = Vec::new();

    loop {
        println!("1. タスクを追加");
        println!("2. タスクを表示");
        println!("3. タスクを削除");
        println!("4. 終了");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        match input {
            "1" => {
                println!("タスクを追加");
                println!("タスクを入力: ");
                let mut task = String::new();
                io::stdin().read_line(&mut task).unwrap();
                let task = task.trim().to_string();
                todo_list.push(task);
            }
            "2" => {
                println!("タスクを表示");
                for task in todo_list.iter() {
                    println!("- {}", task);
                }
            }
            "3" => {
                println!("タスクを削除");
                println!("タスクの番号を選んでください");
                for (index, task) in todo_list.iter().enumerate() {
                    println!("{}: {}", index, task);
                }
                println!("タスクの番号を入力: ");
                let mut task = String::new();
                io::stdin().read_line(&mut task).unwrap();
                let target = task.trim().parse::<usize>().unwrap();
                todo_list.remove(target);
            }
            "4" => {
                println!("終了");
                break;
            }
            _ => {
                println!("無効な入力");
            }
        }
    }
}
