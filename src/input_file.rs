use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, Error};

use super::{recipie::Recipie,production::Production,ressource::Ressource,target::Target};


#[derive(Deserialize, Serialize, Debug,Clone)]
pub struct InputFile {
    #[serde(default)]
    pub resources: HashMap<String,Ressource>,
    #[serde(default)]
    pub recepies: HashMap<String,Recipie>,
    #[serde(default)]
    pub targets: HashMap<String,Target>,
    #[serde(default)]
    pub productions: HashMap<String,Production>,
}

impl InputFile {
    pub fn write_to_file(&self, path: &String) -> Result<(), Error> {
        let json_data = match serde_json::to_string_pretty(self) {
            Ok(res) => res,
            Err(error) => return Err(Error::from(error)),
        };
        let mut input_file = match File::create(&path) {
            Ok(file) => file,
            Err(error) => return Err(error),
        };
        match input_file.write_all(&json_data.as_bytes()) {
            Ok(_) => {}
            Err(error) => return Err(error),
        };
        Ok(())
    }
}
