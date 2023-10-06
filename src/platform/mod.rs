pub mod kahoot;

use anyhow::Result;
use async_trait::async_trait;
use thirtyfour::WebDriver;

#[async_trait]
pub trait Platform {
    async fn get_question(driver: &WebDriver) -> Result<String>;
    async fn get_possible_answers(driver: &WebDriver) -> Result<Vec<String>>;
    async fn choose_answer(index: usize, driver: &WebDriver) -> Result<()>;
}
