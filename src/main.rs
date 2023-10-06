#![feature(lazy_cell)]
mod answer_provider;
mod platform;

use answer_provider::{random::Random, AnswerProvider};
use anyhow::Result;
use platform::{kahoot::Kahoot, Platform};
use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let caps = DesiredCapabilities::firefox();

    let driver = WebDriver::new("http://127.0.0.1:4444", caps).await?;

    driver.goto("https://kahoot.it").await?;
    loop {
        let Ok(question) = Kahoot::get_question(&driver).await else {
            eprintln!("Get question error");
            continue;
        };
        eprintln!("DEBUGPRINT[1]: main.rs:18: question={:#?}", question);
        let possible_answers = Kahoot::get_possible_answers(&driver).await?;
        eprintln!("DEBUGPRINT[2]: main.rs:22: possible_answers={:#?}", possible_answers);
        let answer_index = Random::get_answer(&question, &possible_answers);
        eprintln!("DEBUGPRINT[3]: main.rs:24: answer_index={:#?}", answer_index);
        Kahoot::choose_answer(answer_index, &driver).await?;
    }

    // driver.quit().await?;

    Ok(())
}
