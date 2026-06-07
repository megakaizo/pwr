use std::fs;
use std::error::Error;

use crate::lenovo::conservation::read_conservation;


const CHARGE_STATUS: &str = "/sys/class/power_supply/BAT0/status";
const CAPACITY: &str = "/sys/class/power_supply/BAT0/capacity";
const MANUFACTURER: &str = "/sys/class/power_supply/BAT0/manufacturer";
const MODEL_NAME: &str = "/sys/class/power_supply/BAT0/model_name";
const ENERGY_CURRENT: &str = "/sys/class/power_supply/BAT0/energy_full";
const ENERGY_DESIGN: &str = "/sys/class/power_supply/BAT0/energy_full_design";
const CYCLE_COUNT: &str = "/sys/class/power_supply/BAT0/cycle_count";
const ENERGY_NOW: &str = "/sys/class/power_supply/BAT0/energy_now";
const POWER_DRAW: &str = "/sys/class/power_supply/BAT0/power_now";
const VOLTAGE: &str = "/sys/class/power_supply/BAT0/voltage_now";
const TECHNOLOGY: &str = "/sys/class/power_supply/BAT0/technology";



pub struct BatteryInfo {
    charge_status: String,
    capacity: u8,
    manufacturer: String,
    model: String,
    energy_design: u32,
    energy_current: u32,
    cycle_count: i32,
    energy_now: u32,
    power_draw: i32,
    voltage: i32,
    technology: String,
    conservation: Option<bool>,
}

impl BatteryInfo {
    fn read_param(path: &str) -> Result<String, Box<dyn Error>> {
        let param = fs::read_to_string(path)?;
        return Ok(param) 
    }
    
    fn read_conservation() -> Option<bool> {
        let conservation = read_conservation();
        match conservation {
            Err(_) => None,
            Ok(c) => Some(c),
        }

    }

    pub fn read_info() -> Result<Self, Box<dyn Error>> {
        let charge_status = Self::read_param(CHARGE_STATUS)?;
        let capacity: u8 = Self::read_param(CAPACITY)?.parse()?;
        let manufacturer = Self::read_param(MANUFACTURER)?;
        let model = Self::read_param(MODEL_NAME)?;
        let technology = Self::read_param(TECHNOLOGY)?;
        let energy_design: u32 = Self::read_param(ENERGY_DESIGN)?.parse()?;
        let energy_current: u32 = Self::read_param(ENERGY_CURRENT)?.parse()?;
        let energy_now: u32 = Self::read_param(ENERGY_NOW)?.parse()?;
        let cycle_count: i32 = Self::read_param(CYCLE_COUNT)?.parse()?;
        let power_draw: i32 = Self::read_param(POWER_DRAW)?.parse()?;
        let voltage: i32 = Self::read_param(VOLTAGE)?.parse()?;
        
        let conservation = Self::read_conservation(); 
        
        return Ok( Self {
            charge_status, 
            capacity, 
            manufacturer, 
            model, 
            energy_design, 
            energy_current, 
            cycle_count, 
            energy_now, 
            power_draw, 
            voltage, 
            technology, 
            conservation,
        } )
    }   
}
