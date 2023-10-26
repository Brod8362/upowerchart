extern crate upower_dbus;
mod history;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let (charge, power) = history::get_history(String::from(""), 100)?;
    println!("{:?} {:?}", charge.last(), power.last());
    Ok(())
}
