use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

use super::{ressource::RessourceUsed, recipie::ProductionStep};

#[derive(Deserialize, Serialize, Debug,Clone)]
pub struct Target {
    #[serde(default)]
    pub inputs: Vec<RessourceUsed>,
    #[serde(default)]
    pub outputs:Vec<RessourceUsed>,
    #[serde(default)]
    pub result:HashMap<String,f32>,
}

impl Target {
    
pub fn process(&mut self,production_line:&mut Vec<ProductionStep>){
    self.result = HashMap::new();

        for input in self.inputs.clone() {
            self
                .result
                .entry(input.ressource.clone())
                .and_modify(|res| {
                    *res += input.amount;
                })
                .or_insert(input.amount);

            for step in production_line.into_iter() {
                let mut factor = f32::INFINITY;

                for input_res in &step.inputs {
                    let usable = self
                        .result
                        .entry(input_res.ressource.clone())
                        .or_default();
                    if factor > (*usable / input_res.amount) {
                        factor = *usable / input_res.amount;
                    }
                }
                for input_res in &step.inputs {
                    self
                        .result
                        .entry(input_res.ressource.clone())
                        .and_modify(|res| {
                            *res -= input_res.amount * factor;
                        });
                }
                for output_res in &step.outputs {
                    self
                        .result
                        .entry(output_res.ressource.clone())
                        .and_modify(|res| {
                            *res += output_res.amount * factor;
                        })
                        .or_insert(output_res.amount * factor);
                }
            }
        }
}
}
