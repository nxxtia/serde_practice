use serde::{Serialize, Deserialize};
use serde_json;

use std::fs::File;
use std::io::Read;

use toml::to_string as to_toml;

#[derive(Serialize, Deserialize, Debug)]
struct PublicTariff {
    id: u32,
    price: u32,
    duration: String,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PrivateTariff {
    client_price: u32,
    duration: String,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Stream {
    user_id: String,
    is_private: bool,
    settings: u32,
    shared_url: String,
    public_tariff: PublicTariff,
    private_tariff: PrivateTariff,
}

#[derive(Serialize, Deserialize, Debug)]
enum RequestType{
    success,
}

#[derive(Serialize, Deserialize, Debug)]
struct Gift {
    id: u32,
    price: u32,
    description: String,

}

#[derive(Serialize, Deserialize, Debug)]
struct Debug {
    duration: String,
    at: String,

}

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    r#type: RequestType,
    stream: Stream,
    gifts: Vec<Gift>,
    debug: Debug
}

fn main() -> Result<(), serde_json::Error> {

    let mut file = File::open("/Users/nastyamillow/Desktop/Rust/serde_practice/src/request.json").unwrap();
    let mut json_str = String::new();
    file.read_to_string(&mut json_str).unwrap();

    let request: Request = serde_json::from_str(&json_str).unwrap();
    let toml_str = to_toml(&request).unwrap();

    dbg!(toml_str);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}