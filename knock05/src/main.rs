use chrono::{DateTime, Local, Utc};

fn main() {
    let utc: DateTime<Utc> = Utc::now();
    println!("UTC: {}", utc);
    println!("UTC: {}", utc.format("%Y-%m-%d %H:%M:%S"));
    let local: DateTime<Local> = Local::now();
    println!("Local: {}", local);
    println!("Local: {}", local.format("%Y-%m-%d %H:%M:%S"));
}
