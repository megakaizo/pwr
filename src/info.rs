use crate::battery::BatteryInfo;


pub fn print_info() {
    let info: BatteryInfo = BatteryInfo::read_info().unwrap();

    println!("Battery");
    println!("────────────────────────");
    println!("Charge:        {}%", info.capacity);
    println!("Status:        {}", info.charge_status);
    println!();
    println!("Health:        {:.2}%", info.energy_current / info.energy_design * 100);
    println!("Cycles:        {}", info.cycle_count);
    println!();
    println!(
        "Energy:        {:.2} Wh / {:.2} Wh",
        info.energy_now,
        info.energy_design
    );
    println!("Manufacturer:  {}", info.manufacturer);
    println!("Model:         {}", info.model);
    println!("Technology:    {}", info.technology);
    println!();
    println!(
        "Conservation:  {}",
        if let Some(_) = info.conservation {
            "ON"
        } else {
            "OFF"
        }
    );
}

