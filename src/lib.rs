#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;
pub mod galaxia;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use schema::galaxies;
use models::{ NewGalaxy, GalaxyData, Galaxy};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn insert_galaxy<'a>(conn: &SqliteConnection, g: GalaxyData) -> usize {
  let owner = web3::helpers::serialize(&g.owner);
  let owner: &str = owner.as_str().unwrap();
  // TODO: Find better way to convert U256 to i64
  let radix: u32 = 16;
  let supply = web3::helpers::serialize(&g.supply);
  let supply = supply.as_str().unwrap();
  let supply: i64 = i64::from_str_radix(&supply[2..], radix).unwrap();
  let gateway: &str = &g.gateway;
  println!{"[INFO] creating galaxy (lib.rs)"}
  let new_galaxy = NewGalaxy {
      total_supply: &supply,
      galaxia_owner: owner,
      gateway
  };

  diesel::insert_into(galaxies::table)
      .values(&new_galaxy)
      .execute(conn)
      .expect("Error saving new galaxy")
  }

pub fn get_galaxy<'a>(connection: &SqliteConnection) {
    use schema::galaxies::dsl::*;
    let results = galaxies
    .limit(1)
    .load::<Galaxy>(connection)
    .expect("Error loading galaxy data");
    for res in results {
        println!("test result {:?}", res);
    }
}