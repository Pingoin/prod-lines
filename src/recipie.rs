
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Recipie {
    #[serde(default)]
    pub id: u32,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub inputs: Vec<RessourceUsed>,
    #[serde(default)]
    pub outputs: Vec<RessourceUsed>,
    #[serde(default)]
    pub duration: f32,
    #[serde(default)]
    pub modifier: f32,
}

#[derive(Deserialize, Serialize, Debug)]

pub struct Production {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub factor: f32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Ressource {
    #[serde(default)]
    pub id: u32,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub value: f32,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct RessourceUsed {
    #[serde(default)]
    pub ressource: u32,
    #[serde(default)]
    pub amount: f32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Target {
    #[serde(default)]
    pub ressource: u32,
    #[serde(default)]
    pub input: bool,
    #[serde(default)]
    pub amount: f32,
}
