use anyhow::Result;
use reqwest::Url;
use serde_derive::Deserialize;
use serde_json::Value;
use raylib::prelude::*;

#[derive(Deserialize)]
struct Advice {
    slip: Value,
}

fn show_msg(text: &str) {
    let w = 640;
    let h = 480;

    let (mut rl, thread) = raylib::init()
        .size(w, h)
        .title("Take One")
        .build();

    let font = rl
        .load_font(&thread, "./Iosevka/Iosevka Nerd Font Complete.ttf")
        .expect("could not load font");

    let text_width = measure_text_ex(&font, text, 20.0, 2.0);
    // println!("{:?}", text_width);

    let half_width: f32 = (w as f32 / 2.0) - text_width.x / 2.0;
    let half_height: f32 = (h as f32 / 2.0) - text_width.y / 2.0;
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        d.draw_text_ex(&font,
                       text,
                       Vector2::new(half_width, half_height),
                       30.0,
                       2.0,
                       Color::WHITE);
    }
}

#[tokio::main]
async fn main() -> Result<()> {

    let url = Url::parse("https://api.adviceslip.com/advice")?;
    let res = reqwest::get(url).await?;

    if res.status() == 200 {
        let text = res.text().await?;
        let advice: Advice = serde_json::from_str(&text[..])?;
        let text = format!("Today's wisdom:\n{}", advice.slip["advice"]);
        show_msg(&text[..]);
        // println!("{}", res);
    } else {
        println!("Oops... Something went wrong...");
    }

    return Ok(());
}
