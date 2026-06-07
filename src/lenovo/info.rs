use std::error::Error;
use std::fs;


const CHARGE_STATUS: &str = "/sys/class/power_supply/BAT0/status";
const CAPACITY: &str = "/sys/class/power_supply/BAT0/capacity";
const MANUFACTURER: &str = "/sys/class/power_supply/BAT0/manufacturer";
const MODEL_NAME: &str = "/sys/class/power_supply/BAT0/model_name";
const HEALTH_CURRENT: &str = "/sys/class/power_supply/BAT0/energy_full";
const HEALTH_DESIGN: &str = "/sys/class/power_supply/BAT0/energy_full_design";
const CYCLES_COUNT: &str = "/sys/class/power_supply/BAT0/cycle_count";
const ENERGY_CURRENT: &str = "/sys/class/power_supply/BAT0/energy_now";
const POWER_DRAW: &str = "/sys/class/power_supply/BAT0/power_now";
const VOLTAGE: &str = "/sys/class/power_supply/BAT0/voltage_now";
const TECHNOLOGY: &str = "/sys/class/power_supply/BAT0/technology";
