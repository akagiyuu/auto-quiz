use std::time::Duration;

use super::Platform;
use anyhow::{anyhow, Result};
use chromiumoxide::{Browser, Page};
use futures::{stream::FuturesUnordered, TryStreamExt};
use tokio::time::sleep;

pub struct Kahoot {
    prev_question: String,
    page: Page,
}

impl Kahoot {
    pub async fn new(browser: &Browser) -> Result<Self> {
        let page = browser.new_page("about:blank").await?;
        page.enable_stealth_mode().await?;
        page.goto("https://kahoot.it").await?;
        Ok(Self {
            prev_question: String::new(),
            page,
        })
    }
}

impl Platform for Kahoot {
    async fn get_question(&mut self) -> Result<String> {
        loop {
            let question = self
                .page
                .find_element("span[data-functional-selector='block-title']")
                .await?
                .inner_text()
                .await?
                .ok_or(anyhow!("No question"))?;
            if question == self.prev_question {
                sleep(Duration::from_secs(1)).await;
                continue;
            }

            self.prev_question = question;
            break;
        }

        Ok(self.prev_question.clone())
    }

    async fn get_possible_answers(&self) -> Result<Vec<String>> {
        let elements = self
            .page
            .find_elements("span[data-functional-selector*='question-choice-text]")
            .await?;
        let possible_answers = elements
            .into_iter()
            .map(|element| async move { element.inner_text().await.map(|x| x.unwrap_or_default()) })
            .collect::<FuturesUnordered<_>>()
            .try_collect::<Vec<_>>()
            .await?;

        Ok(possible_answers)
    }

    async fn choose_answer(&self, index: usize) -> Result<()> {
        let css_selector = format!("button[data-functional-selector='answer-{}']", index);
        let answer_button = self.page.find_element(css_selector).await?;
        answer_button.click().await?;
        Ok(())
    }
}
