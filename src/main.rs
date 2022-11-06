use std::collections::HashMap;

use prod_lines::{
    read_from_file,
    recipie::{IOtype, ProductionStep},
};

fn main() -> std::io::Result<()> {
    let path = String::from("production.json");
    let mut data = read_from_file(&path);

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
    
    //let mut result:HashMap<String,f32>=HashMap::new();

    for target in data.targets.values_mut(){
        for input in target.inputs.clone(){
            target.result.entry(input.ressource.clone())
            .and_modify(|res|{
                *res+=input.amount;
            }).or_insert(input.amount);

            for step in & mut production_line{
                let mut factor=f32::INFINITY;

                for input_res in &step.inputs{
                    let usable=target.result.entry(input_res.ressource.clone()).or_default();
                    if factor>(*usable/input_res.amount){
                        factor=*usable/input_res.amount;
                    }
                }
                for input_res in &step.inputs{
                    target.result.entry(input_res.ressource.clone()).and_modify(|res|{
                        *res-= input_res.amount * factor;
                    });
                }
                for output_res in &step.outputs {
                    target.result.entry(output_res.ressource.clone()).and_modify(|res|{
                        *res+= output_res.amount * factor;
                    }).or_insert(output_res.amount * factor);
                }
            }
        }
    }
    dbg!(&data.targets);
    data.write_to_file(&path)
}
