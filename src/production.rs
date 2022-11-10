use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug,Clone)]
pub struct Production {
    #[serde(default)]
    pub name: String,
    #[serde(default = "float1")]
    pub factor: f32,
    #[serde(default)]
    pub recipes: Vec<String>,
    #[serde(default = "default_true")]
    #[serde(rename = "devideRecipies")]
    pub devide_recipes: bool,
}

fn float1() -> f32 {
    1.0
}

fn default_true() -> bool {
    true
}