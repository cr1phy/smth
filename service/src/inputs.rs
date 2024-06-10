use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct UserInput {
    pub name: String,
}
