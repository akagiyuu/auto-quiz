use super::AnswerProvider;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use anyhow::Result;

#[derive(Serialize)]
struct Prompt {
    text: String
}
#[derive(Serialize)]
struct PalmRequest {
    prompt: Prompt
}

#[derive(Deserialize, Debug)]
struct Candidate {
    output: String,
}
#[derive(Deserialize, Debug)]
struct PalmResponse {
    candidates: Vec<Candidate>,
}

pub struct Palm {
    endpoint: String
}
impl Default for Palm {
    fn default() -> Self {
        Self { 
            endpoint: format!(
                "https://generativelanguage.googleapis.com/v1beta3/models/text-bison-001:generateText?key={}",
                env::var("PALM_API_KEY").unwrap()
            ) 
        }
    }
}
impl Palm {
    fn get_request_data(&self, question: &str, possible_answers: &[String]) -> PalmRequest {
        let mut answers = String::new();
        for (i, answer) in possible_answers.iter().enumerate() {
            answers.push_str(&format!("{}: {}\n", i, answer));
        }

        PalmRequest {
            prompt: Prompt { text: format!(
                r#"
                Please choose the best answer to the following question and only respond with the number of the answer:

                Question: 1 + 2?
                0: 3
                1: 8
                2: 7
                3: 0
                Answer: 0

                Question: 8 - 4?
                0: 3
                1: 8
                2: 4
                3: 0
                Answer: 2

                Question: {}
                {}
                Answer:"#, question, answers)
            }
        }
    }
}

impl AnswerProvider for Palm {
    async fn get_answer(&self, question: &str, possible_answers: &[String]) -> Result<usize> {
        let prompt_data = self.get_request_data(question, possible_answers);

        let client = Client::new();
        let response = client
            .post(&self.endpoint)
            .json(&prompt_data)
            .send().await?
            .json::<PalmResponse>().await?;
        Ok(response.candidates[0].output.parse::<usize>()?)
    }
}
