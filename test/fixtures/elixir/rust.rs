// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

extern crate serde_json;

#[derive(Serialize, Deserialize)]
pub struct Pokedex {
    #[serde(rename = "pokemon")]
    pokemon: Vec<Pokemon>,
}

#[derive(Serialize, Deserialize)]
pub struct Pokemon {
    #[serde(rename = "id")]
    id: i64,

    #[serde(rename = "num")]
    num: String,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "img")]
    img: String,

    #[serde(rename = "type")]
    pokemon_type: Vec<Type>,

    #[serde(rename = "height")]
    height: String,

    #[serde(rename = "weight")]
    weight: String,

    #[serde(rename = "candy")]
    candy: String,

    #[serde(rename = "candy_count")]
    candy_count: Option<i64>,

    #[serde(rename = "egg")]
    egg: Egg,

    #[serde(rename = "spawn_chance")]
    spawn_chance: f64,

    #[serde(rename = "avg_spawns")]
    avg_spawns: f64,

    #[serde(rename = "spawn_time")]
    spawn_time: String,

    #[serde(rename = "multipliers")]
    multipliers: Option<Vec<f64>>,

    #[serde(rename = "weaknesses")]
    weaknesses: Vec<Type>,

    #[serde(rename = "next_evolution")]
    next_evolution: Option<Vec<Evolution>>,

    #[serde(rename = "prev_evolution")]
    prev_evolution: Option<Vec<Evolution>>,
}

#[derive(Serialize, Deserialize)]
pub struct Evolution {
    #[serde(rename = "num")]
    num: String,

    #[serde(rename = "name")]
    name: String,
}

#[derive(Serialize, Deserialize)]
pub enum Egg {
    #[serde(rename = "Not in Eggs")]
    NotInEggs,

    #[serde(rename = "Omanyte Candy")]
    OmanyteCandy,

    #[serde(rename = "10 km")]
    The10Km,

    #[serde(rename = "2 km")]
    The2Km,

    #[serde(rename = "5 km")]
    The5Km,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Bug")]
    Bug,

    #[serde(rename = "Dark")]
    Dark,

    #[serde(rename = "Dragon")]
    Dragon,

    #[serde(rename = "Electric")]
    Electric,

    #[serde(rename = "Fairy")]
    Fairy,

    #[serde(rename = "Fighting")]
    Fighting,

    #[serde(rename = "Fire")]
    Fire,

    #[serde(rename = "Flying")]
    Flying,

    #[serde(rename = "Ghost")]
    Ghost,

    #[serde(rename = "Grass")]
    Grass,

    #[serde(rename = "Ground")]
    Ground,

    #[serde(rename = "Ice")]
    Ice,

    #[serde(rename = "Normal")]
    Normal,

    #[serde(rename = "Poison")]
    Poison,

    #[serde(rename = "Psychic")]
    Psychic,

    #[serde(rename = "Rock")]
    Rock,

    #[serde(rename = "Steel")]
    Steel,

    #[serde(rename = "Water")]
    Water,
}
