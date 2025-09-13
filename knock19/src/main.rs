use chrono::{Datelike, Days, Local, NaiveDate};
use crossterm::style::{Color, ResetColor, SetForegroundColor};
use std::env;
use std::io::{Write, stdout};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 今日の日付
    let today = Local::now().date_naive();

    // 引数として 年と月を取得
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        return Err("年と月を指定してください".into());
    }

    let year: i32 = args[1].parse()?;
    let month: u32 = args[2].parse()?;

    // 次の月の1日を作る
    let next_month: u32 = if month == 12 { 1 } else { month + 1 };
    let next_month_year: i32 = if month == 12 { year + 1 } else { year };
    let first_day_next_month: NaiveDate = NaiveDate::from_ymd_opt(next_month_year, next_month, 1)
        .ok_or("次の月の1日を作ることができません")?;

    // この月の1日を作る
    let first_day: NaiveDate =
        NaiveDate::from_ymd_opt(year, month, 1).ok_or("この月の1日を作ることができません")?;
    let last_day: NaiveDate = first_day_next_month
        .pred_opt()
        .ok_or("次の月の1日前を作ることができません")?;

    let first_weekday = first_day.weekday().num_days_from_sunday();
    let mut current_day = first_day;
    let mut current_weekday = first_weekday;

    println!(" Sun Mon Tue Wed Thu Fri Sat");

    for _ in 0..first_weekday {
        print!("    ");
    }

    let mut stdout = stdout();

    while current_day <= last_day {
        if current_day == today {
            // 今日なら赤にする
            write!(
                stdout,
                "{}{:>4}{} ",
                SetForegroundColor(Color::Red),
                current_day.day(),
                ResetColor
            )?;
        } else {
            write!(stdout, "{:>4}", current_day.day())?;
        }

        current_day = current_day
            .checked_add_days(Days::new(1))
            .ok_or("日付を加算できません")?;
        current_weekday += 1;

        if current_weekday == 7 {
            println!();
            current_weekday = 0;
        }
    }

    println!();

    Ok(())
}
