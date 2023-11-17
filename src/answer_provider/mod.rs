pub mod gpt;
pub mod random;
pub mod palm;
use anyhow::Result;

pub trait AnswerProvider {
    async fn get_answer(&self, question: &str, possible_answers: &[String]) -> Result<usize>;
}
