use reqwest;
use serde::Deserialize;

// deriveはrustのコンパイラに以下のtraitを自動実装するよう命令するための記述
#[derive(Deserialize, Debug)]
struct ForecastResponse {
    forecasts: Vec<Forecast>,
}

#[derive(Deserialize, Debug)]
struct Forecast {
    date: String,
    telop: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 天気予報APIを叩き、情報を取得する
    let body: ForecastResponse =
        reqwest::get("https://weather.tsukumijima.net/api/forecast/city/140020")
            .await?
            .json()
            .await?;

    // body.forecastsはVecという可変長配列、Vecはイテレータに変換可能なのでforを使える
    for forecast in body.forecasts {
        println!("{}: {}", forecast.date, forecast.telop);
    }
    Ok(())
}
