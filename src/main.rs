use std::{error::Error};

use argparse::{ArgumentParser, Store};

mod history;

fn main() -> Result<(), Box<dyn Error>> {
    let mut device: String = String::new();
    let mut width: u32 = 300;
    let mut height: u32 = 200;

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("generate graph of battery history using upower");
        ap.refer(&mut device)
            .required()
            .add_option(&["-d", "--device"], Store, "device model to use");
        ap.refer(&mut width)
            .add_option(&["-w"], Store, "window width");
        ap.refer(&mut height)
            .add_option(&["-h"], Store, "window height");
        ap.parse_args_or_exit();
    }

    let (charge, power) = history::get_history(&device)?;

    history::generate_graph(&charge, &power, &device)?;
    
    Ok(())
}
