use pyo3::{
    types::{PyDict, PyModule},
    Python,
};

use super::AnswerProvider;

pub struct GPT;

impl AnswerProvider for GPT {
    fn get_answer(question: &str, possible_answers: &[String]) -> usize {
        let mut compact_question = String::from(question);
        compact_question.push('?');
        for possible_answer in possible_answers {
            compact_question.push('\n');
            compact_question.push_str(possible_answer);
        }
        let mut answer_index = Python::with_gil(|py| {
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
        }).unwrap().parse::<usize>().unwrap();

        if answer_index > 0 {
            answer_index -= 1;
        }
        println!("Answer index: {}", answer_index);
        answer_index
    }
}
