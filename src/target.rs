use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

use crate::ressource::RessourceUsed;

#[derive(Deserialize, Serialize, Debug,Clone)]
pub struct Target {
    #[serde(default)]
    pub inputs: Vec<RessourceUsed>,
    #[serde(default)]
    pub outputs:Vec<RessourceUsed>,
    #[serde(default)]
    pub result:HashMap<String,f32>,
}
