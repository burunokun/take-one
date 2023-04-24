use anyhow::Result;
use reqwest::Url;
use serde_derive::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
struct Advice {
    slip: Value,
}

#[tokio::main]
async fn main() -> Result<()> {

    let url = Url::parse("https://api.adviceslip.com/advice")?;
    let res = reqwest::get(url).await?;

    if res.status() == 200 {
        let text = res.text().await?;
        let advice: Advice = serde_json::from_str(&text[..])?;
        let res = format!("Today's wisdom: {}", advice.slip["advice"]);
        println!("{}", res);
    } else {
        println!("Oops... Something went wrong...");
    }

    return Ok(());

}
