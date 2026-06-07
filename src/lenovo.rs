use std::error::Error;
use std::fs;

const CONSERVATION_MODE: &str = "/sys/bus/platform/drivers/ideapad_acpi/VPC2004:00/conservation_mode";


fn set_conservation(mode: bool) -> Result<(), Box<dyn Error>> {
    match mode {
        true => fs::write(CONSERVATION_MODE, "1")?,
        false => fs::write(CONSERVATION_MODE, "0")?,
    }
    Ok(())
}


fn read_conservation() -> Result<bool, Box<dyn Error>> {
    let mode = fs::read_to_string(CONSERVATION_MODE)?;
    match mode.as_str() {
        "1" => Ok(true),
        "0" => Ok(false),
        other => Err(format!("Invalid conservation mode: {other}").into()),
    }
}
