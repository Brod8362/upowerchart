use std::error::Error;

use plotters::style::RGBColor;

pub struct RenderConfig {
    // Device model in upower - e.g 45N1029
    pub device: String,
    // Window width
    pub width: i32,
    // Window height
    pub height: i32,
    // Hour range to display
    pub hours: i32,
    // Label area size
    pub label_area_size: i32,
    // Graph margin
    pub graph_margin: i32,
    // Extra margin at bottom (for extra text)
    pub bottom_margin_extra: i32,
    // Axis rendering color
    pub axis_color: String,
    // Image background color
    pub background_color: String,
    // Percent graph color (also used for percent text)
    pub percent_color: String,
    // Charging grpah color 
    pub charging_color: String,
    // Discharging graph color (also used for power text)
    pub discharging_color: String,
}

pub fn parse_color(s: &str) -> Result<RGBColor, Box<dyn Error>> {
    let chars = s.trim_start_matches("#");
    let r = u8::from_str_radix(&chars[0..2], 16)?;
    let g = u8::from_str_radix(&chars[2..4], 16)?;
    let b = u8::from_str_radix(&chars[4..6], 16)?;
    
    Ok(RGBColor(r,g,b))
}