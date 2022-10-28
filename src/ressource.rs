use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug,Clone)]
pub struct Ressource {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub value: f32,
}

#[derive(Deserialize, Serialize, Debug,Clone)]
pub struct RessourceUsed {
    #[serde(default)]
    pub ressource: String,
    #[serde(default)]
    pub amount: f32,
}