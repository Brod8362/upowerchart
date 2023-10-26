use std::{error::Error};

extern crate upower_dbus;
mod history;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (charge, power) = history::get_history(String::from("45N1029"))?;
    // println!("{:?} {:?}", charge.last(), power.last());

    history::generate_graph(&charge, &power)?;
    
    Ok(())
}
