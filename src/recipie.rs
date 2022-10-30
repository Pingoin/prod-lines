use super::ressource::RessourceUsed;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
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

impl Recipie {

    pub fn to_production_step(&self) -> ProductionStep {
        ProductionStep {
            inputs: self.inputs.clone(),
            outputs: self.outputs.clone(),
            production_capacity: 0.0,
            duration:self.duration,
        }
    }

}

pub enum IOtype {
    Input,
    Output,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProductionStep {
    pub inputs: Vec<RessourceUsed>,
    pub outputs: Vec<RessourceUsed>,
    pub duration:f32,
    pub production_capacity: f32,
}

impl ProductionStep {
    pub fn add_output_to_list(&self, list: &mut Vec<String>) {
        self.outputs.iter().for_each(|prod| {
            let production = prod.ressource.clone();
            if !list.contains(&production) {
                list.push(production)
            }
        });
    }

    pub fn list_contains(&self, input_output: IOtype, list: &Vec<String>) -> bool {
        let mut result = false;
        let put_list;

        match input_output {
            IOtype::Input => put_list = &self.inputs,
            IOtype::Output => put_list = &self.outputs,
        }

        put_list.iter().for_each(|prod| {
            if list.contains(&prod.ressource) {
                result = true;
            }
        });

        result
    }
}