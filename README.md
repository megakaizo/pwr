# PWR
Minimalistic Linux CLI tool for check battery info and manage battery conservation status


`pwr info` command output example:

```
Battery
────────────────────────
Charge:        79%
Status:        Not charging

Health:        99.72%
Cycles:        126

Energy:        47.56 Wh / 60.00 Wh
Power Draw:    0.00 W
Voltage:       12.23 V

Manufacturer:  ATL
Model:         L24N3PK1
Technology:    Li-poly

Conservation:  ON
```

## Features

- Battery charge percentage & status
- Health, cycle count, energy, voltage, power draw
- Manufacturer and model info
- Conservation mode (enable battery limit charge) — currently Lenovo Ideapad laptops only

## Installation

```
cargo install --git https://github.com/megakaizo/pwr
```

or

```
git clone git@github.com:megakaizo/pwr.git

cd pwr

cargo build --release

sudo cp target/release/pwr /usr/local/bin/
```


## Usage

```
# Show battery info
pwr info

# Enable conservation mode (requires sudo)
sudo pwr conservation on

# Disable conservation mode (requires sudo)
sudo pwr conservation off
```

## Requirements

- Linux with /sys/class/power_supply/
- ideapad_acpi driver for conservation mode (Lenovo Ideapad):
- root for conservation mode changes
for check:
```
ls /sys/bus/platform/drivers/ideapad_acpi/VPC2004:00/conservation_mode
```
if file exists, conservation mode switch is enable


## How it works

Reads battery data from sysfs (/sys/class/power_supply/) 
and writes to the ideapad_acpi driver interface (/sys/bus/platform/drivers/ideapad_acpi/) to toggle conservation mode.

