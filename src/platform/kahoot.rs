use std::time::Duration;

use super::Platform;
use anyhow::Result;
use thirtyfour::{prelude::ElementQueryable, By, WebDriver};

pub struct Kahoot {
    prev_question: String,
    driver: WebDriver,
}
impl From<WebDriver> for Kahoot {
    fn from(driver: WebDriver) -> Self {
        Self {
            prev_question: String::new(),
            driver,
        }
    }
}

impl Platform for Kahoot {
    async fn get_question(&mut self) -> Result<String> {
        loop {
            let question = self
                .driver
                .query(By::Css("span[data-functional-selector='block-title']"))
                .wait(Duration::from_secs(600), Duration::from_secs(3))
                .first()
                .await?
                .text()
                .await?;
            if question == self.prev_question {
                continue;
            }

            self.prev_question = question;
            break;
        }
        Ok(self.prev_question.clone())
    }

    async fn get_possible_answers(&self) -> Result<Vec<String>> {
        let mut i = 0usize;
        let mut posible_answers = vec![];
        loop {
            let css_selector = format!(
                "span[data-functional-selector='question-choice-text-{}']",
                i
            );
            match self.driver.find(By::Css(css_selector.as_str())).await {
                Err(_) => break,
                Ok(answer_element) => {
                    posible_answers.push(answer_element.text().await?);
                }
            }
            i += 1;
        }
        Ok(posible_answers)
    }

    async fn choose_answer(&self, index: usize) -> Result<()> {
        let css_selector = format!("button[data-functional-selector='answer-{}']", index);
        let answer_button = self.driver.find(By::Css(css_selector.as_str())).await?;
        answer_button.click().await?;
        Ok(())
    }
}
