use super::Step;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Strap {
    pub name: String,
    pub context: Option<String>,
    pub steps: Vec<Step>,
}
