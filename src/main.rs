pub mod watch;
pub mod facility;
pub mod target;

use ethers_contract_abigen::Abigen;

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let facility_cfg = facility::read_facility_cfg("./mock-cfg.json")?;

//     println!("cfg: {:?} \n", facility_cfg);

//     let fetcher = target::Fetcher::new(&facility_cfg);
//     let state = fetcher.fetch_ibport_state();

//     println!("state: {:?} \n", state);

//     Ok(())
// }
fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    Abigen::new("LUPort", "./abi/luport.json")?.generate()?.write_to_file("luport_abi.rs")?;

    Ok(())
}
