#![feature(lazy_cell)]
#![feature(never_type)]
mod answer_provider;
mod platform;

use answer_provider::{palm::Palm, AnswerProvider};
use anyhow::Result;
use platform::{kahoot::Kahoot, Platform};
use thirtyfour::prelude::*;

#[tokio::main]
async fn main() -> Result<!> {
    tracing_subscriber::fmt::init();

    let capabilities = DesiredCapabilities::firefox();
    let driver = WebDriver::new("http://127.0.0.1:4444", capabilities).await?;
    driver.goto("https://kahoot.it").await?;

    let palm = Palm::default();
    let mut kahoot = Kahoot::from(driver);
    loop {
        let question = match kahoot.get_question().await {
            Ok(question) => question,
            Err(error) => {
                tracing::error!("Get question error: {}", error);
                continue;
            }
        };
        let possible_answers = match kahoot.get_possible_answers().await {
            Ok(question) => question,
            Err(error) => {
                tracing::error!("Get possible answers error: {}", error);
                continue;
            }
        };
        let answer = match palm.get_answer(&question, &possible_answers).await {
            Ok(question) => question,
            Err(error) => {
                tracing::error!("Get answer error: {}", error);
                continue;
            }
        };
        if let Err(error) = kahoot.choose_answer(answer).await {
            tracing::error!("Choose answer error: {}", error);
            continue;
        }
    }
}
