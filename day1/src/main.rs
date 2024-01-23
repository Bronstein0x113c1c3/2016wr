use std::collections::HashMap;
type Table = HashMap<String, Vec<String>>;
fn show(table: &mut Table) {
    table.insert(
        "Leonardo DaVinci".to_string(),
        vec!["L'Ultima Cena".to_string()],
    );
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!(" {}", work);
        }
    }
}
fn other_table(table: &mut Table) {
    // table.insert("Leonardo DaVinci".to_string(), vec!["Salvator Mundi".to_string()]);
    match table.get_mut("Leonardo DaVinci") {
        Some(res) => {
            res.push("Salvator Mundi".to_string());
        }
        None => {
            println!("Couldn't find him....");
        }
    };
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!(" {}", work);
        }
    }
}
fn main() {
    let mut table = Table::new();
    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );
    table.insert(
        "Caravaggio".to_string(),
        vec![
            "The Musicians".to_string(),
            "The Calling of St. Matthew".to_string(),
        ],
    );
    table.insert(
        "Cellini".to_string(),
        vec![
            "Perseus with the head of Medusa".to_string(),
            "a salt cellar".to_string(),
        ],
    );
    show(&mut table);
    other_table(&mut table);
}
