use std::{cmp::max, error::Error, borrow::{BorrowMut, Borrow}};

use minifb::{Window, MouseButton, Key, WindowOptions};
use plotters::{prelude::{BitMapBackend, ChartBuilder, LabelAreaPosition, IntoDrawingArea}, backend::BGRXPixel, style::{TextStyle, IntoTextStyle, Color}, series::{AreaSeries, LineSeries}};

use crate::{history::HistoryEntry, buffer::BufferWrapper, config::{RenderConfig, parse_color}};

pub fn render_graph(charge: &[HistoryEntry], power: &[HistoryEntry], config: &RenderConfig) -> Result<(), Box<dyn Error>> {
    let width: i32 = config.width;
    let height: i32 = config.height;
    let label_area_size: i32 = config.label_area_size;
    let graph_margin: i32 = config.graph_margin;
    let bottom_margin_extra: i32 = config.bottom_margin_extra;

    //style settings
    let axis_color = parse_color(&config.axis_color)?;
    let background_color = parse_color(&config.background_color)?;
    let percent_color = parse_color(&config.percent_color)?;
    let charging_color = parse_color(&config.charging_color)?;
    let discharging_color = parse_color(&config.discharging_color)?;
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
    let mut window_options = WindowOptions::default();
    window_options.borderless = true;
    window_options.topmost = true;
    let window_title = format!("upowerchart - {}", &config.device);
    let mut window = Window::new(&window_title, width as usize, height as usize, window_options)?;
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
            (config.device.to_string(), &text_style),
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