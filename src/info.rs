use std::error::Error;
use std::fs;

use crate::{battery::BatteryInfo, lenovo::conservation::{
    self, read_conservation, set_conservation 
}};


pub fn print_info() {
    let info: BatteryInfo = BatteryInfo::read_info().unwrap();
    let conservation = read_conservation();
    match conservation {
        Ok() => 
    } 
}

