use web3::contract::{Options, Contract};
use web3::futures::Future;
use web3::types::{U256, Address};
use std::env;
use dotenv::dotenv;
use super::models::{GalaxyData, GalaxiaContract, ERC721, Metadata, Asset};

impl GalaxiaContract<web3::transports::Http> {

  pub fn get_galaxy_data(&self) -> GalaxyData {
    let owner: Address = self.contract.query("owner", (), None, Options::default(), None).wait().unwrap();
    let total_supply = self.contract.query("totalSupply", (), None, Options::default(), None);
    let total_supply: U256 = total_supply.wait().unwrap();
    let gateway: String = self.contract.query("ipfsGateway", (), None, Options::default(), None).wait().unwrap();
    GalaxyData { supply: total_supply, owner: owner, gateway: gateway }
  }

  pub async fn get_metadata(&self, token_id: U256) -> Result<Metadata, Box<dyn std::error::Error>> {
    let galaxia: &str= &env::var("GALAXIA_SERVER").expect("Galaxia server not set in .env");
    let hash: String = self.contract.query("idToUri", token_id, None, Options::default(), None).wait().unwrap();
    let hash: &str = hash.as_str();
    let uri: String = format!("{}{}", galaxia, hash);
    let metadata: Metadata = reqwest::get(&uri).await?.json().await?;
    Ok(metadata)
  }

  pub async fn get_all_assets(&self) -> Vec<Asset> {
    let supply: U256 = self.contract.query("totalSupply", (), None, Options::default(), None).wait().unwrap();
    let number_assets: u128 = supply.as_u128();
    let mut assets: Vec<Asset> = Vec::new();
    for id in 0..number_assets {
      let token_id: U256 = id.into();
      let owner: Address = self.contract.query("ownerOf", token_id, None, Options::default(), None).wait().unwrap();
      let token_uri: String = self.contract.query("idToUri", token_id, None, Options::default(), None).wait().unwrap();
      let erc721: ERC721 = ERC721 { owner: owner, id: token_id, uri: token_uri };
      // TODO handle error and retry x times + try different ipfs server
      let metadata: Metadata = self.get_metadata(token_id).await.unwrap();
      let asset: Asset = Asset { erc_data: erc721, metadata: metadata };
      assets.push(asset);
    }
    assets
  }
}

pub fn connect_contract() -> GalaxiaContract<web3::transports::Http> {
  dotenv().ok();
  let ropsten: &str = &env::var("INFURA").expect("Infura node url not set");
  let (_eloop, transport) = web3::transports::Http::new(ropsten).unwrap();
  _eloop.into_remote();
  let web3 = web3::Web3::new(transport);
  let contract_address: Address = "c9ee3f436989DF09a57E5b446265ad412cdC1CBB".parse().unwrap();
  let contract = Contract::from_json(
      web3.eth(),
      contract_address,
      include_bytes!("../src/build/Galaxia.json"),
  )
  .unwrap();
  GalaxiaContract { contract: contract, web3: web3 }
}