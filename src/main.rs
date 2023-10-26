use std::error::Error;

use argparse::{ArgumentParser, Store};
use config::RenderConfig;

mod history;
mod buffer;
mod config;
mod render;

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
            .add_option(&["-w"], Store, "window width [default 300]");
        ap.refer(&mut config.height)
            .add_option(&["-h"], Store, "window height [default 200]");
        ap.refer(&mut config.hours)
            .add_option(&["-t"], Store, "display range (hours) [default 3]");
        ap.refer(&mut config.label_area_size)
            .add_option(&["--label-area-size"], Store, "label area size [default 20]");
        ap.refer(&mut config.graph_margin)
            .add_option(&["--graph-margin"], Store, "graph margin [default 10]");
        ap.refer(&mut config.bottom_margin_extra)
            .add_option(&["--bottom-margin-extra"], Store, "bottom margin extra [default 10]");
        ap.refer(&mut config.axis_color)
            .add_option(&["-a", "--axis-color"], Store, "axis color [default #FFFFFF]");
        ap.refer(&mut config.background_color)
            .add_option(&["-b", "--background-color"], Store, "background color [default #000000]");
        ap.refer(&mut config.percent_color)
            .add_option(&["-p", "--percent-color"], Store, "percent color [default #00FF00]");
        ap.refer(&mut config.charging_color)
            .add_option(&["-c", "--charging-color"], Store, "charging color [default #00FFFF]");
        ap.refer(&mut config.discharging_color)
            .add_option(&["-x", "--discharging-color"], Store, "discharging color [default #FF8800]");
        ap.parse_args_or_exit();
    }

    let (charge, power) = history::get_history(&config.device)?;

    render::render_graph(&charge, &power, &config)?;
    
    Ok(())
}
