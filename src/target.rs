use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug,Clone)]
pub struct Target {
    #[serde(default)]
    pub ressource: String,
    #[serde(default)]
    pub input: bool,
    #[serde(default)]
    pub amount: f32,
}
