use std::error::Error;

use pokedex::Pokemon;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let json = reqwest::get("https://pokeapi.co/api/v2/pokemon/5").await?.text().await?;
    let mon: Pokemon = serde_json::from_str(&json)?;
    println!("{:?}", mon);
    Ok(())
}
