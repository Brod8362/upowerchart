use std::{path::Path, error::Error, cmp::max};

use plotters::{prelude::{BitMapBackend, IntoDrawingArea, ChartBuilder, LabelAreaPosition}, series::LineSeries, style::{full_palette::{ORANGE}, RGBColor, TextStyle, IntoTextStyle}};

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

pub fn get_history(device_name: String) -> Result<(Vec<HistoryEntry>, Vec<HistoryEntry>), Box<dyn Error>> {
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

fn convert_entry(entry: &HistoryEntry) -> (i32, i32) {
    (entry.time as i32, entry.value.round() as i32)
}

pub fn generate_graph(charge: &[HistoryEntry], power: &[HistoryEntry]) -> Result<(), Box<dyn Error>> {
    let width = 300;
    let height = 200;
    let label_area_size = 20;
    let graph_margin = 10;

    //style settings
    let axis_color = RGBColor(255,255,255);
    let background_color = RGBColor(0,0,0);
    let charge_color = RGBColor(0,255,0);
    let rate_color = ORANGE;
    let font = ("monospace", 12);

    let hours: i32 = 3;

    //filter by data within the past 6 hours
    let last_timestamp = max(charge.last().unwrap().time, power.last().unwrap().time);
    let first_timestamp = last_timestamp - (60*60*hours as u64); //6 hours

    //convert data into time-series, relative to hours
    let charge_series = charge.iter()
        .filter(|x| x.time > first_timestamp)
        .map(|x| convert_entry(x));

    let rate_series = power.iter()
        .filter(|x| x.time > first_timestamp)
        .map(|x| convert_entry(x));

    let drawing_area = BitMapBackend::new("output.png", (width, height)).into_drawing_area();
    drawing_area.fill(&background_color)?;

    let text_style = TextStyle::from(font)
        .color(&axis_color)
        .into_text_style(&drawing_area);

    let time_range = first_timestamp as i32..last_timestamp as i32;

    let mut charge_chart = ChartBuilder::on(&drawing_area)
        .margin(graph_margin)
        .set_label_area_size(LabelAreaPosition::Left, label_area_size)
        .set_label_area_size(LabelAreaPosition::Bottom, label_area_size)
        .build_cartesian_2d(time_range.clone(), 0..100)
        .unwrap();

    let rate_max = power.iter().map(|x| x.value).max_by(|x, y| x.total_cmp(y)).unwrap().ceil() as i32;

    let mut rate_chart = ChartBuilder::on(&drawing_area)
        .margin(graph_margin)
        .set_label_area_size(LabelAreaPosition::Left, label_area_size)
        .set_label_area_size(LabelAreaPosition::Bottom, label_area_size)
        .build_cartesian_2d(time_range, 0..rate_max)
        .unwrap();

    // this is only used to draw a custom axis
    let mut time_chart = ChartBuilder::on(&drawing_area)
        .margin(graph_margin)
        .set_label_area_size(LabelAreaPosition::Left, label_area_size)
        .set_label_area_size(LabelAreaPosition::Bottom, label_area_size)
        .build_cartesian_2d(-hours..0, 0..100)
        .unwrap();

    //draws X axis with the marks being hours
    //draws Y axis with 10 marks (being for battery %)
    time_chart
        .configure_mesh()
        .disable_mesh()
        .axis_style(&axis_color)
        .x_desc("hours")
        .x_labels(hours as usize)
        .label_style(text_style)
        .draw()?;

    //TODO: draw right Y axis for power

    //draw the actual data
    charge_chart.draw_series(LineSeries::new(charge_series, charge_color))?;
    rate_chart.draw_series(LineSeries::new(rate_series, rate_color))?;
    Ok(())
}