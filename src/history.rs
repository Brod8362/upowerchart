use std::{path::Path, error::Error};

use cairo::{ImageSurface, Context};

const UPOWER_PATH: &str = "/var/lib/upower";

#[derive(Debug)]
pub struct HistoryEntry {
    time: u64,
    value: f64,
    charging: bool
}

pub fn parse_file(path: &Path) -> Result<Vec<HistoryEntry>, Box<dyn Error>> {
    let file = std::fs::read_to_string(path).unwrap();
    Ok(
        file.lines().map(|l| {
            let mut groups = l.split('\t');
            HistoryEntry {
                time: groups.next().unwrap().parse().unwrap(),
                value: groups.next().unwrap().parse().unwrap(),
                charging: groups.next().unwrap() == "charging"
            }
        }).collect()
    )
}

pub fn get_history(device_name: String, len: usize) -> Result<(Vec<HistoryEntry>, Vec<HistoryEntry>), Box<dyn Error>> {
    let upower_entries = std::fs::read_dir(UPOWER_PATH)?;
    let mut charge_filename: Option<String> = None;
    let mut rate_filename: Option<String> = None;

    let charge_pattern = format!("history-charge-{}", device_name);
    let rate_pattern = format!("history-rate-{}", device_name);
    for x in upower_entries {
        if let Ok(t) = x {
            if let Ok(s) = t.file_name().into_string() {
                if charge_filename.is_none() && s.starts_with(&charge_pattern) {
                    charge_filename = Some(s)
                } else if rate_filename.is_none() && s.starts_with(&rate_pattern) {
                    rate_filename = Some(s)
                }
            }
        }
    }
    //TODO don't assume they were found
    let charge_path_str = format!("{}/{}", UPOWER_PATH, charge_filename.unwrap());
    let rate_path_str = format!("{}/{}", UPOWER_PATH, rate_filename.unwrap());
    let charge_path = Path::new(&charge_path_str);
    let rate_path = Path::new(&rate_path_str);

    let charge = parse_file(charge_path)?;
    let rate = parse_file(rate_path)?;
    Ok((charge, rate))
}

pub fn generate_graph(charge: &[HistoryEntry], power: &[HistoryEntry]) -> Result<ImageSurface, Box<dyn Error>> {
    let width = 600;
    let height = 400;
    let surface = ImageSurface::create(cairo::Format::ARgb32, width, height)?;
    let context = Context::new(&surface)?;

    //TODO these should be passed in via some kind of config object
    let background_color = (0.0, 0.0, 0.0);
    let battery_color = (0.0, 1.0, 0.0);

    // fill background
    context.set_source_rgb(background_color.0, background_color.1, background_color.2);
    context.paint()?;

    
    let max_power = power
        .into_iter()
        .map(|k| k.value)
        .max_by(|x, y| x.total_cmp(y))
        .unwrap();

    //draw battery percentage 
    {
        let max_charge = 100.0f64;
        context.set_source_rgb(battery_color.0, battery_color.1, battery_color.2);
        let x_step = width as f64/charge.len() as f64;
        for i in 1..charge.len() {
            let prev = &charge[i-1];
            let this = &charge[i];
            let prev_x = (i-1) as f64*x_step;
            let x = i as f64*x_step;
            let prev_y = prev.value/max_charge;
            let y = height as f64*(this.value/max_charge);
            context.move_to(prev_x, height as f64 - prev_y);
            context.line_to(x, height as f64 - y);
        }
        context.stroke()?;
    }
    
    
    Ok(surface)
}