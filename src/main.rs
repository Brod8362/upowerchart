use std::{error::Error, str::FromStr};

use argparse::{ArgumentParser, Store};
use config::RenderConfig;
use plotters::style::RGBColor;

mod history;
mod buffer;
mod config;

fn main() -> Result<(), Box<dyn Error>> {
    let mut config = RenderConfig {
        device: "".to_string(),
        width: 300,
        height: 200,
        hours: 3,
        label_area_size: 20,
        graph_margin: 10,
        bottom_margin_extra: 10,
        axis_color: "#FFFFFF".to_string(),
        background_color: "#000000".to_string(),
        percent_color: "#00FF00".to_string(),
        charging_color: "#00FFFF".to_string(),
        discharging_color: "#FF8000".to_string(),
    };

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("generate graph of battery history using upower");
        ap.refer(&mut config.device)
            .required()
            .add_option(&["-d", "--device"], Store, "device model to use");
        ap.refer(&mut config.width)
            .add_option(&["-w"], Store, "window width");
        ap.refer(&mut config.height)
            .add_option(&["-h"], Store, "window height");
        ap.parse_args_or_exit();
    }

    let (charge, power) = history::get_history(&config.device)?;

    history::generate_graph(&charge, &power, &config)?;
    
    Ok(())
}
