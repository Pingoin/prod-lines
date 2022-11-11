use prod_lines::{
    create_production_line, get_wighted_recipes, input_file::InputFile, read_from_file,
};

fn main() -> std::io::Result<()> {
    let path = String::from("production.json");
    let (ressources, recipes, mut target, productions) = read_from_file(&path);

    let wighted_recipies = get_wighted_recipes(&productions, &recipes);

    let production_line = create_production_line(&wighted_recipies);
    dbg!(&production_line);

    target.process(&production_line);

    dbg!(&target);
    InputFile {
        ressources,
        recipes,
        target,
        productions,
    }
    .write_to_file(&path)
}
