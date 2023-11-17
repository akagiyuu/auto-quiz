pub mod kahoot;

use anyhow::Result;

pub trait Platform {
    async fn get_question(&mut self) -> Result<String>;
    async fn get_possible_answers(&self) -> Result<Vec<String>>;
    async fn choose_answer(&self, index: usize) -> Result<()>;
}
