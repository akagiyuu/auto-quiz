#![allow(unstable_name_collisions)]
use std::iter;

use anyhow::{Error, Result};
use itertools::Itertools;
use ollama_rs::generation::completion::request::GenerationRequest;

pub async fn get_answer(question: &str, possible_answers: &[String]) -> Result<usize> {
    let ollama = ollama_rs::Ollama::default();

    let model = "quiz".to_string();
    let prompt = iter::once(question)
        .chain(possible_answers.iter().map(|x| x.as_str()))
        .intersperse("\n")
        .collect::<String>();

    let result = ollama
        .generate(GenerationRequest::new(model, prompt))
        .await?;

    result.response.parse().map_err(Error::from)
}
