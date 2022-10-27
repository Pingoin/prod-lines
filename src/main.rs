use std::collections::HashMap;

use prod_lines::recipie::{read_from_file, Recipie};

fn main() -> std::io::Result<()> {
    let path = String::from("production.json");
    let mut data =read_from_file(&path);
    let mut result:HashMap<String,Recipie>=HashMap::new();

    data.productions.iter_mut().for_each(|(_,prod)|{
        let index =prod.recipies[0].clone();
        let recipie = data.recepies[&index].clone();

        result.insert(index, recipie);
    });
    dbg!(result);
    data.write_to_file(&path)
}
