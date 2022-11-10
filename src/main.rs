use prod_lines::{create_production_line, get_wighted_recipies, read_from_file};

fn main() -> std::io::Result<()> {
    let path = String::from("production.json");
    let mut data = read_from_file(&path);

    let wighted_recipies = get_wighted_recipies(&data.productions, &data.recepies);

    let production_line = create_production_line(&wighted_recipies);
dbg!(&production_line);
    for target in data.targets.values_mut() {
        target.process(& production_line)
    }
    dbg!(&data.targets);
    data.write_to_file(&path)
}
