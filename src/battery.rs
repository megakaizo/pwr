pub struct BatteryInfo {
    charge_status: String,
    capacity: u8,
    manufacturer: String,
    model: String,
    health_design: i32,
    health_current: i32,
    cycle_count: i32,
    energy_current: i32,
    power_draw: i32,
    voltage: i32,
    technology: String,
    conservation_mode: Option<bool>,
}
