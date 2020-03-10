use std::convert::TryFrom;
use std::fmt;
use std::fmt::Display;
use std::fs;
use std::path::{Path, PathBuf};

use libbar::{Module, ModuleOutput};

use rood::{Cause, CausedResult, Error};

enum BatteryValue {
    String(String),
    Int(i32),
}

impl BatteryValue {
    pub fn expect_int(&self) -> CausedResult<i32> {
        match self {
            BatteryValue::Int(i) => Ok(*i),
            BatteryValue::String(s) => Err(Error::new(
                Cause::InvalidData,
                &format!("Invalid battery int data: {}", s),
            )),
        }
    }

    pub fn expect_string(&self) -> CausedResult<String> {
        match self {
            BatteryValue::String(s) => Ok(String::from(s)),
            BatteryValue::Int(i) => Err(Error::new(
                Cause::InvalidData,
                &format!("Invalid battery string data: {}", i),
            )),
        }
    }
}

enum BatteryStatus {
    Charging,
    Discharging,
    Full,
}

impl Display for BatteryStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BatteryStatus::Charging => "Charging",
                BatteryStatus::Full => "Full",
                BatteryStatus::Discharging => "Discharging",
            }
        )
    }
}

impl TryFrom<String> for BatteryStatus {
    type Error = Error;

    fn try_from(v: String) -> CausedResult<BatteryStatus> {
        match v.as_ref() {
            "Full" => Ok(BatteryStatus::Full),
            "Charging" => Ok(BatteryStatus::Charging),
            "Discharging" => Ok(BatteryStatus::Discharging),
            _ => Err(Error::new(
                Cause::InvalidData,
                &format!("Invalid battery status: {}", v),
            )),
        }
    }
}

struct BatteryInfo {
    path: PathBuf,
}

impl BatteryInfo {
    pub fn new<P: AsRef<Path>>(p: P) -> BatteryInfo {
        let path = PathBuf::from(p.as_ref());
        BatteryInfo { path }
    }

    fn key(&self, key: &str) -> CausedResult<BatteryValue> {
        let battery_file = self.path.join(key);
        let raw_data = fs::read_to_string(&battery_file)?.trim().to_string();
        Ok(match raw_data.parse::<i32>() {
            Ok(f_val) => BatteryValue::Int(f_val),
            Err(_) => BatteryValue::String(raw_data),
        })
    }

    pub fn consumption(&self) -> CausedResult<f32> {
        let voltage_now = self.key("voltage_now")?.expect_int()?;
        let current_now = self.key("current_now")?.expect_int()?;
        let consumption = voltage_now as f32 * current_now.abs() as f32;

        Ok(if consumption > 0.1 { consumption } else { 0.0 })
    }

    pub fn status(&self) -> CausedResult<BatteryStatus> {
        match BatteryStatus::try_from(self.key("status")?.expect_string()?) {
            Ok(stat) => Ok(stat),
            Err(_) => {
                // Fallback on percentage for charging status.
                if self.consumption()? > 0.0 && self.percentage()? < 100.0 {
                    Ok(BatteryStatus::Charging)
                } else {
                    Err(Error::new(
                        Cause::InvalidData,
                        "Unknown battery charge state",
                    ))
                }
            }
        }
    }

    pub fn percentage(&self) -> CausedResult<f32> {
        let charge_now = self.key("charge_now")?.expect_int()?;
        let voltage_now = self.key("voltage_now")?.expect_int()?;
        let remaining: f32 = charge_now as f32 * voltage_now as f32;

        let charge_full = self.key("charge_full")?.expect_int()?;

        Ok(remaining / (charge_full as f32 * voltage_now as f32) * 100.0)
    }
}

pub struct Battery {
    info: BatteryInfo,
}

impl Battery {
    pub fn new() -> Battery {
        let info = BatteryInfo::new("/sys/class/power_supply/BAT0"); // TODO: Detect batteries.
        Battery { info }
    }

    pub fn boxed(self) -> Box<Battery> {
        Box::new(self)
    }
}

impl Module for Battery {
    fn name(&self) -> &'static str {
        "battery"
    }

    fn output(&mut self) -> CausedResult<ModuleOutput> {
        Ok(ModuleOutput::new("battery").with_full_text(format!(
            "± {:.1}% ({})",
            self.info.percentage()?,
            self.info.status()?
        )))
    }
}
