use prod_lines::{create_production_line, get_wighted_recipes, read_from_file, input_file::InputFile};

fn main() -> std::io::Result<()> {
    let path = String::from("production.json");
    let (ressources,recipes,mut targets,productions) = read_from_file(&path);

    let wighted_recipies = get_wighted_recipes(&productions, &recipes);

    let production_line = create_production_line(&wighted_recipies);
dbg!(&production_line);
    for target in targets.values_mut() {
        target.process(& production_line)
    }
    dbg!(&targets);
    InputFile{
        ressources,recipes,targets,productions
    }.write_to_file(&path)
}
