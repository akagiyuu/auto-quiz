mod answer_provider;
mod platform;

use answer_provider::ollama;
use anyhow::Result;
use chromiumoxide::{Browser, BrowserConfig};
use futures::StreamExt;
use platform::{kahoot::Kahoot, Platform};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let (browser, mut handler) =
        Browser::launch(BrowserConfig::builder().with_head().build().unwrap()).await?;

    // spawn a new task that continuously polls the handler
    tokio::task::spawn(async move {
        loop {
            let _ = handler.next().await;
        }
    });

    let mut kahoot = Kahoot::new(&browser).await?;
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
        let answer = match ollama::get_answer(&question, &possible_answers).await {
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
