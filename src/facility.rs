use std::fs::File;
use std::io::prelude::*;

use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

// use arrayref::array_ref;
use bs58;


pub fn pubkey_to_bytes(pubkey: &str) -> Pubkey {
    let bytes = bs58::decode(pubkey).into_vec().unwrap();

    // let bytes = pubkey.from_base58().unwrap();
    // let bytes = array_ref![bytes.as_slice(), 0, 32];

    // Pubkey::new_from_array(*bytes)
    Pubkey::new(bytes.as_slice())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FacilityConfig {
    pub node_url: String,
    pub ibport_program_id: String,
    pub ibport_data_account: String,
}

pub fn read_facility_cfg(path: &str) -> Result<FacilityConfig, Box<dyn std::error::Error>> {
    // let mut file = File::open("foo.txt")?;
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // assert_eq!(contents, "Hello, world!");
    let cfg: FacilityConfig = serde_json::from_str(contents.as_str())?;

    Ok(cfg)
}