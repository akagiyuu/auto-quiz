use rand::Rng;

use super::AnswerProvider;
use anyhow::Result;

#[derive(Default)]
pub struct Random;

impl AnswerProvider for Random {
    async fn get_answer(&self, _question: &str, possible_answers: &[String]) -> Result<usize> {
        Ok(rand::thread_rng().gen_range(0..possible_answers.len()))
    }
}
