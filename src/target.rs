use std::error;

use solana_sdk::program_pack::Pack;
use solana_client::rpc_client::RpcClient;

use solana_sdk::{
    pubkey::Pubkey,
    commitment_config::CommitmentConfig,
};
use std::{
    collections::{btree_map::Entry, BTreeMap},
    str::FromStr,
};

use crate::facility;

use solana_ibport_contract::ibport::state::IBPortContract;


pub struct Fetcher {
    client: RpcClient,
    cfg: facility::FacilityConfig,
}

impl Fetcher {
    pub fn new(cfg: &facility::FacilityConfig) -> Fetcher {
        Fetcher {
            // client: Client::default()
            client: RpcClient::new_with_commitment(cfg.node_url.clone(), CommitmentConfig::confirmed()),
            cfg: cfg.clone(),
        }
    }
    
    fn fetch_ibport_state_encoded(&self) -> Result<Vec<u8>, Box<dyn error::Error>> {
        match self.client.get_account_data(
            &facility::pubkey_to_bytes(self.cfg.ibport_data_account.as_str())
        ) {
            Ok(v) => Ok(v),
            Err(e) => Err(Box::new(e))
        }
    }

    pub fn fetch_ibport_state(&self) -> Result<IBPortContract, Box<dyn error::Error>> {
        match self.fetch_ibport_state_encoded() {
            Ok(encoded) => {
                let ibport_contract_info = IBPortContract::unpack_from_slice(encoded.as_slice())?;
                Ok(ibport_contract_info)
            }
            Err(e) => Err(e)
        }
    }
}

