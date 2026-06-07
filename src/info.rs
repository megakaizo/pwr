use crate::battery::BatteryInfo;


pub fn print_info() {
    let info: BatteryInfo = BatteryInfo::read_info().unwrap();

    println!("Battery");
    println!("────────────────────────");
    println!("Charge:        {}%", info.capacity);
    println!("Status:        {}", info.charge_status);
    println!();
    println!("Health:        {:.2}%", (info.energy_current as f64 / info.energy_design as f64) * 100.0);
    println!("Cycles:        {}", info.cycle_count);
    println!();
    println!(
        "Energy:        {:.2} Wh / {:.2} Wh",
        info.energy_now as f64 / 1000000.0,
        info.energy_design as f64 / 1000000.0
    );
    println!("Power Draw:    {:.2} W", info.power_draw as f64 / 1000000.0);
    println!("Voltage:       {:.2} V", info.voltage as f64 / 1000000.0);
    println!();
    println!("Manufacturer:  {}", info.manufacturer);
    println!("Model:         {}", info.model);
    println!("Technology:    {}", info.technology);
    println!();
    println!(
        "Conservation:  {}",
        if let Some(con) = info.conservation_mode {
            if con {
                "ON"
            } else {
                "OFF"
            }
        } else {
            ""
        }
    );
}

