use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::{fs::File, io::Read};

#[derive(Debug, Deserialize)]
struct StoryArc {
    title: String,
    story: Vec<String>,
    options: Vec<StoryOpt>,
}

#[derive(Debug, Deserialize)]
struct StoryOpt {
    text: String,
    arc: String,
}

fn main() {
    let mut file = File::open("gopher.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading");
    let json: Value = serde_json::from_str(&contents).expect("Formatting error");

    let map_arcs: HashMap<_, _> = json
        .as_object()
        .unwrap()
        .to_owned()
        .into_iter()
        .map(|e| {
            (
                e.0,
                serde_json::from_value::<StoryArc>(e.1).expect("Bad StoryArc"),
            )
        })
        .collect();

    println!("Map: {:#?}", map_arcs);
}
