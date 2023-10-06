use std::{
    sync::Mutex,
    time::Duration,
};

use super::Platform;
use anyhow::Result;
use async_trait::async_trait;
use std::sync::LazyLock;
use thirtyfour::{prelude::ElementQueryable, By, WebDriver};


static QUESTION: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new("".to_string()));

pub struct Kahoot;

#[async_trait]
impl Platform for Kahoot {
    async fn get_question(driver: &WebDriver) -> Result<String> {
        loop {
            let question = driver
                .query(By::Css("span[data-functional-selector='block-title']"))
                .wait(Duration::from_secs(600), Duration::from_secs(3))
                .first()
                .await?
                .text()
                .await?;
            if question == *QUESTION.lock().unwrap() {
                continue;
            }
            
            *QUESTION.lock().unwrap() = question.clone();
            return Ok(question);
        };
    }

    async fn get_possible_answers(driver: &WebDriver) -> Result<Vec<String>> {
        let mut i = 0usize;
        let mut posible_answers = vec![];
        loop {
            let css_selector = format!(
                "span[data-functional-selector='question-choice-text-{}']",
                i
            );
            match driver.find(By::Css(css_selector.as_str())).await {
                Err(_) => break,
                Ok(answer_element) => {
                    posible_answers.push(answer_element.text().await?);
                }
            }
            i += 1;
        }
        Ok(posible_answers)
    }

    async fn choose_answer(index: usize, driver: &WebDriver) -> Result<()> {
        let css_selector = format!("button[data-functional-selector='answer-{}']", index);
        let answer_button = driver.find(By::Css(css_selector.as_str())).await?;
        answer_button.click().await?;
        Ok(())
    }
}
