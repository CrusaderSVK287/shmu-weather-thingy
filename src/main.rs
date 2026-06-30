#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let body = shmu_get_dates()
    .await?;

    println!("{body}");

    Ok(())
}

async fn shmu_get_dates() -> Result<String, reqwest::Error> {
    let body = reqwest::get("http://opendata.shmu.sk/meteorology/weather/alerts/cap/")
        .await?
        .text()
        .await?;

    Ok(body)
}