use rand::Rng;

use anyhow::Result;

pub async fn get_answer(_question: &str, possible_answers: &[String]) -> Result<usize> {
    Ok(rand::thread_rng().gen_range(0..possible_answers.len()))
}
