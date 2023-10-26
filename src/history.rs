use std::{path::Path, error::Error, cmp::max, borrow::{BorrowMut, Borrow}};

use minifb::{Window, WindowOptions, Key, MouseButton};
use plotters::{prelude::{BitMapBackend, IntoDrawingArea, ChartBuilder, LabelAreaPosition}, series::{LineSeries, AreaSeries}, style::{full_palette::{ORANGE}, RGBColor, TextStyle, IntoTextStyle, Color, CYAN}, backend::BGRXPixel};

use crate::buffer::BufferWrapper;

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

pub fn get_history(device_name: &String) -> Result<(Vec<HistoryEntry>, Vec<HistoryEntry>), Box<dyn Error>> {
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

pub fn generate_graph(charge: &[HistoryEntry], power: &[HistoryEntry], model: &String) -> Result<(), Box<dyn Error>> {
    let width: i32 = 300;
    let height: i32 = 200;
    let label_area_size: i32 = 20;
    let graph_margin: i32 = 10;
    let bottom_margin_extra: i32 = 10;

    //style settings
    let axis_color = RGBColor(255,255,255);
    let background_color = RGBColor(0,0,0);
    let percent_color = RGBColor(0,255,0);
    let charging_color = CYAN;
    let discharging_color = ORANGE;
    let font = ("monospace", 12);

    let hours: i32 = 3;

    //filter by data within the past 6 hours
    let last_timestamp = max(charge.last().unwrap().time, power.last().unwrap().time);
    let first_timestamp = last_timestamp - (60*60*hours as u64); //6 hours

    //convert data into time-series, relative to hours
    let charge_pct_series = charge.iter()
        .filter(|x| x.time > first_timestamp)
        .map(|entry| (entry.time as i32, entry.value.round() as i32));

    //for when battery is charging
    let chg_rate_series = power.iter()
        .filter(|x| x.time > first_timestamp)
        .map(|entry| {
            let s = if entry.charging {
                entry.value.round() as i32
            } else {
                0i32
            };
            (entry.time as i32, s)
        });

    // for when battery is discharging
    let dischchg_rate_series = power.iter()
        .filter(|x| x.time > first_timestamp && !x.charging)
        .map(|entry| {
            let s = if !entry.charging {
                entry.value.round() as i32
            } else {
                0i32
            };
            (entry.time as i32, s)
        });

    // drawing 
    let mut buf = BufferWrapper(vec![0u32; width as usize*height as usize]);
    let mut window = Window::new("battery", width as usize, height as usize, WindowOptions::default())?;
    window.limit_update_rate(Some(std::time::Duration::from_millis(100)));
    
    {
        let drawing_area = BitMapBackend::<BGRXPixel>::with_buffer_and_format(
            buf.borrow_mut(), (width as u32, height as u32)
        )?.into_drawing_area();
        drawing_area.fill(&background_color)?;        

        let text_style = TextStyle::from(font)
            .color(&axis_color)
            .into_text_style(&drawing_area);

        let time_range = first_timestamp as i32..last_timestamp as i32;

        let mut charge_chart = ChartBuilder::on(&drawing_area)
            .margin(graph_margin)
            .margin_bottom(graph_margin+bottom_margin_extra)
            .set_label_area_size(LabelAreaPosition::Left, label_area_size)
            .set_label_area_size(LabelAreaPosition::Bottom, label_area_size)
            .build_cartesian_2d(time_range.clone(), 0..100)
            .unwrap();

        let rate_max = power.iter().map(|x| x.value).max_by(|x, y| x.total_cmp(y)).unwrap().ceil() as i32;

        let mut rate_chart = ChartBuilder::on(&drawing_area)
            .margin(graph_margin)
            .margin_bottom(graph_margin+bottom_margin_extra)
            .set_label_area_size(LabelAreaPosition::Left, label_area_size)
            .set_label_area_size(LabelAreaPosition::Bottom, label_area_size)
            .build_cartesian_2d(time_range, 0..rate_max)
            .unwrap();

        // this is only used to draw a custom axis
        let mut time_chart = ChartBuilder::on(&drawing_area)
            .margin(graph_margin)
            .margin_bottom(graph_margin+bottom_margin_extra)
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
            .label_style(text_style.clone())
            .draw()?;

        //TODO: draw right Y axis for power

        //draw the actual data
        charge_chart.draw_series(
            AreaSeries::new(
                charge_pct_series, 
                0, 
                percent_color.mix(0.2)
            ).border_style(&percent_color)
        )?;

        let texts = [
            (model.clone(), &text_style),
            (format!("BAT: {}%", charge.last().unwrap().value), &text_style.color(&percent_color)),
            (format!("PWR: {:.1}W", power.last().unwrap().value), &text_style.color(&discharging_color))
        ];

        let (_, text_h) = drawing_area.estimate_text_size("?", &text_style)?;
        let y_pos = (height - text_h as i32- bottom_margin_extra/2) as i32;
        let mut x = 20;
        for (text, style) in texts {
            drawing_area.draw_text(&text, &style, (x, y_pos))?;
            let (tx, _) = drawing_area.estimate_text_size(&text, &style)?;
            x = x + tx as i32 + 5; //5 is text margin
        }
        
        rate_chart.draw_series(LineSeries::new(chg_rate_series, charging_color))?;
        rate_chart.draw_series(LineSeries::new(dischchg_rate_series, discharging_color))?;

        drawing_area.present()?;
    }
    
    while window.is_open() && !(window.get_mouse_down(MouseButton::Left) || window.is_key_down(Key::Escape)) {
        window.update_with_buffer(buf.borrow(), width as usize, height as usize)?;
    }
    Ok(())
}