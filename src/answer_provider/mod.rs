pub mod chatgpt;
pub mod random;

pub trait AnswerProvider {
    fn get_answer(question: &str, possible_answers: &[String]) -> usize;
}
