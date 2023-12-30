use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct City {
    name: String,
    population: u64,
    latitude: f64,
    longitude: f64,
}

fn main() {
    let calabar = City {
        name: String::from("Calabar"),
        population: 474_000,
        latitude: 4.95,
        longitude: 8.33,
    };

    let as_json = serde_json::to_string(&calabar).unwrap();
    println!("as json: {}", as_json);

    let as_cbor = serde_cbor::to_vec(&calabar).unwrap();
    println!("as cbor: {:?}", as_cbor);
    println!("cobr (as UTF-8): {:?}", String::from_utf8_lossy(&as_cbor));

    let as_bincode = bincode::serialize(&calabar).unwrap();
    println!("as bincode: {:?}", as_bincode);
    println!(
        "bincode (as UTF-8): {:?}",
        String::from_utf8_lossy(&as_bincode)
    );
}
