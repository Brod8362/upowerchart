extern crate upower_dbus;

use std::time::Duration;

use upower_dbus::{UPowerProxy, BatteryState};

struct PowerSample {
    percentage: f64, 
    charging: bool,
    power: f64
}

#[tokio::main]
async fn main() -> Result<(), zbus::Error> {
    let connection = zbus::Connection::system().await?;

    let upower = UPowerProxy::new(&connection).await?;

    let mut samples: Vec<PowerSample> = Vec::new();

    loop {
        let device = upower.get_display_device().await?;

        let sample = PowerSample { 
            percentage: device.percentage().await?, 
            charging: device.state().await? == BatteryState::Charging, 
            power: 0.0f64 //TODO: how to get watts/power draw?
        };
        samples.push(sample);

        tokio::time::sleep(Duration::from_secs(10)).await;
    }
    
}
