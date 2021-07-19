pub mod watch;
pub mod facility;
pub mod target;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // println!("Hello, world!");
    // watch::hello();

    let facility_cfg = facility::read_facility_cfg("./mock-cfg.json")?;

    println!("cfg: {:?} \n", facility_cfg);

    let fetcher = target::Fetcher::new(&facility_cfg);
    let state = fetcher.fetch_ibport_state();

    println!("state: {:?} \n", state);

    Ok(())
}
