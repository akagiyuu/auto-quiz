use pyo3::{
    types::{PyDict, PyModule},
    Python,
};

use super::AnswerProvider;
use anyhow::Result;

#[derive(Default)]
pub struct GPT;

impl AnswerProvider for GPT {
    async fn get_answer(&self, question: &str, possible_answers: &[String]) -> Result<usize> {
        let mut compact_question = String::from(question);
        for possible_answer in possible_answers {
            compact_question.push('\n');
            compact_question.push_str(possible_answer);
        }

        let mut answer = Python::with_gil(|py| {
            let g4f = PyModule::import(py, "g4f")?;
            let config = PyDict::new(py);
            config.set_item("role", "system")?;
            config.set_item("content", "You are a helpful assistant specialized in answering multiple-choice questions who only responds with the button number corresponding to the most likely answer do not respond with words only a integer. Remember only respond with an integer between 1 and 4 that corresponds to the answer.")?;
            let messages = PyDict::new(py);
            messages.set_item("role", "user")?;
            messages.set_item("content", compact_question)?;
            let args = PyDict::new(py);
            args.set_item("model", "gpt-3.5-turbo")?;
            args.set_item("messages", [config, messages])?;
            let test = g4f
                .getattr("ChatCompletion")?
                .getattr("create")?
                .call((), Some(args))?;
            test.extract::<String>()
        })?.parse::<usize>()?;

        if answer > 0 {
            answer -= 1;
        }
        Ok(answer)
    }
}
