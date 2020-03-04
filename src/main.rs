#[macro_use]
extern crate web3;
extern crate reqwest;
extern crate serde;
extern crate futures;
extern crate diesel;
extern crate indexer;

use std::env;
use web3::{ 
    contract::{Contract},
    types::{Address}
};
use indexer::{
    galaxia::{connect_contract},
    establish_connection,
    insert_galaxy,
    get_galaxy,
    models::{GalaxyData, Galaxy, GalaxiaContract, Asset}
};


#[tokio::main]
async fn main() {
    let galaxia: GalaxiaContract<web3::transports::Http> = connect_contract();
    let info: GalaxyData = galaxia.get_galaxy_data();
    // println!("{:?}", info);
    // let assets: Vec<Asset> = galaxia.get_all_assets().await;
    // println!("{:?}", assets);
    let connection = establish_connection();
    let _res = insert_galaxy(&connection, info);
    // let owner: Address = "50f06a6fe481eeafcf19c8adbbd9f2f7d312dcd8".parse().unwrap();
    let galaxy_check = get_galaxy(&connection);
}
