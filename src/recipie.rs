use serde_derive::{Deserialize, Serialize};
use super::ressource::RessourceUsed;

#[derive(Deserialize, Serialize, Debug,Clone)]
pub struct Recipie {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub inputs: Vec<RessourceUsed>,
    #[serde(default)]
    pub outputs: Vec<RessourceUsed>,
    #[serde(default)]
    pub duration: f32,
}
