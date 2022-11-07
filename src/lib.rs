pub mod recipie;
pub mod production;
pub mod input_file;
pub mod ressource;
pub mod target;

use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*};

use input_file::InputFile;
use production::Production;
use recipie::{ProductionStep, Recipie, IOtype};

pub fn read_from_file(path: &String) -> InputFile {
    let mut input_file = match File::open(path) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let mut string_data = String::from("");
    let bla = input_file.read_to_string(&mut string_data);
    match bla {
        Ok(_) => {},
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let data = {
        match serde_json::from_str::<InputFile>(&string_data) {
            Ok(result) => result,
            Err(error) => panic!("Problem : {:?}", error),
        }
    };
    data
}

pub fn get_wighted_recipies(productions: &HashMap<String, Production>,recepies: &HashMap<String, Recipie>)->HashMap<String, ProductionStep>{
    let mut wighted_recipies: HashMap<String, ProductionStep> = HashMap::new();
    for (_, production) in productions.iter() {
        for recipie_id in production.recipies.iter() {
            let mut recipie = match wighted_recipies.get_mut(recipie_id) {
                Some(recipie) => recipie.clone(),
                None => match recepies.get(recipie_id) {
                    Some(rec) => rec.to_production_step(),
                    None => ProductionStep::new(),
                },
            };
            recipie.production_capacity += production.factor / recipie.duration;
            wighted_recipies.insert(recipie_id.clone(), recipie);
            
        }
    }
    wighted_recipies
}

pub fn create_production_line(wighted_recipies:&HashMap<String, ProductionStep>)->Vec<ProductionStep>{
    let mut production_line: Vec<ProductionStep> = Vec::new();
    {
        let mut produced_ressources: Vec<String> = Vec::new();
        let mut already_produced_ressources: Vec<String> = Vec::new();
        let mut inserted_keys: Vec<String> = Vec::new();

        for (_, recipie) in wighted_recipies.iter() {
            recipie.add_output_to_list(&mut produced_ressources);
        }

        while production_line.len() < wighted_recipies.len() {
            for (key, recipie) in wighted_recipies.iter() {
                if (!recipie.list_contains(IOtype::Input, &produced_ressources)
                    || recipie.list_contains(IOtype::Input, &already_produced_ressources))
                    && !inserted_keys.contains(key)
                {
                    production_line.push(recipie.clone());
                    inserted_keys.push(key.clone());
                    recipie.add_output_to_list(&mut already_produced_ressources);
                }
            }
        }
    }
production_line
}