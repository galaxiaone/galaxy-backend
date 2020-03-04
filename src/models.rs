extern crate web3;
use web3::contract::{Contract};
use web3::types::{U256, Address};
use web3::Transport;

use serde::{Serialize, Deserialize};
use serde_json::Value;

use diesel::*;
use super::schema::galaxies;

#[derive(Queryable, Debug)]
pub struct Galaxy {
  pub id: i32,
  pub total_supply: i64,
  pub galaxia_owner: String,
  pub gateway: String
}


#[derive(Insertable)]
#[table_name="galaxies"]
pub struct NewGalaxy<'a> {
    pub total_supply: &'a i64,
    pub galaxia_owner: &'a str,
    pub gateway: &'a str,
}

#[derive(Debug)]
pub struct GalaxiaContract<T: Transport> {
  pub web3: web3::Web3<T>,
  pub contract: Contract<T>
}


#[derive(Debug, Serialize, Deserialize)]
pub struct GalaxyData {
  pub supply: U256,
  pub owner: Address,
  pub gateway: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ERC721 {
  pub owner: Address,
  pub id: U256,
  pub uri: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attributes {
  display_type: Option<String>,
  trait_type: String,
  value: Value
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
  pub attributes: Vec<Attributes>,
  pub background_color: String,
  pub description: String,
  pub external_url: String,
  pub image: String,
  pub name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Asset {
  pub erc_data: ERC721,
  pub metadata: Metadata
}
