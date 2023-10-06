use rand::Rng;

use super::AnswerProvider;

pub struct Random;

impl AnswerProvider for Random {
    fn get_answer(question: &str, possible_answers: &[String]) -> usize {
        rand::thread_rng().gen_range(0..possible_answers.len())
    }
}
