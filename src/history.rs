use std::{path::Path, error::Error};

#[derive(Debug)]
pub struct HistoryEntry {
    time: u64,
    value: f64,
    charging: bool
}

pub fn parse_file(path: &Path) -> Result<Vec<HistoryEntry>, ()> {
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

pub fn get_history(device_name: String, len: usize) -> Result<(Vec<HistoryEntry>, Vec<HistoryEntry>), ()> {
    let charge = parse_file(Path::new("/var/lib/upower/history-charge-45N1029-86-18880.dat"))?;
    let rate = parse_file(Path::new("/var/lib/upower/history-rate-45N1029-86-18880.dat"))?;
    Ok((charge, rate))
}

pub fn generate_graph(charge: &[HistoryEntry], power: &[HistoryEntry]) {
    todo!()
}