use std::{path::Path, error::Error};

const UPOWER_PATH: &str = "/var/lib/upower";

#[derive(Debug)]
pub struct HistoryEntry {
    pub time: u64,
    pub value: f64,
    pub charging: bool
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