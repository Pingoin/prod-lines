use std::collections::HashMap;

use prod_lines::{
    read_from_file,
    recipie::{IOtype, ProductionStep},
};

fn main() -> std::io::Result<()> {
    let path = String::from("production.json");
    let data = read_from_file(&path);

    let mut wighted_recipies: HashMap<String, ProductionStep> = HashMap::new();
    for (key, val) in data.recepies.iter() {
        wighted_recipies.insert(key.clone(), val.to_production_step());
    }

    for (_, production) in data.productions.iter() {
        for recipie_id in production.recipies.iter() {
            wighted_recipies.entry(recipie_id.clone()).and_modify(|f| {
                f.production_capacity += production.factor / f.duration;
            });
        }
    }
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
    
    
    data.write_to_file(&path)
}
