use std::env;

fn main() {
    // 環境変数を受け取る
    let env = match env::var("EDITOR") {
        Ok(env) => env,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };
    println!("EDITOR: {}", env);

    let envs = env::vars();
    for (key, value) in envs {
        println!("{}: {}", key, value);
    }
}
