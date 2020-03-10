use std::fs;

use rood::{Cause, CausedResult, Error};

use libbar::{Module, ModuleOutput};

const LOAD_FILE: &str = "/proc/loadavg";
const OK_COLOR: &str = "#FFFFFF";
const CRIT_COLOR: &str = "#FF0000";

pub struct Load {
    alert_threshold: f32,
    ok_color: String,
    critical_color: String,
}

impl Load {
    pub fn new() -> Load {
        Load {
            alert_threshold: 6.0, // TODO: Get nb. of cores.
            ok_color: String::from(OK_COLOR),
            critical_color: String::from(CRIT_COLOR),
        }
    }

    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    pub fn with_ok_color(mut self, ok_color: String) -> Self {
        self.ok_color = ok_color;
        self
    }

    pub fn with_critical_color(mut self, crit_color: String) -> Self {
        self.critical_color = crit_color;
        self
    }

    pub fn with_alert_threshold(mut self, threshold: f32) -> Self {
        self.alert_threshold = threshold;
        self
    }

    fn get_load_avg() -> CausedResult<(f32, f32)> {
        let raw = fs::read_to_string(LOAD_FILE)?;
        let parts: Vec<String> = raw.split(" ").map(|r| String::from(r)).collect();

        if parts.len() < 2 {
            return Err(Error::new(
                Cause::InvalidData,
                "Invalid data for load average",
            ));
        }

        let min_avg: f32 = parts[0]
            .parse::<f32>()
            .map_err(|e| Error::new(Cause::InvalidData, &e.to_string()))?;

        let fivemin_avg: f32 = parts[1]
            .parse::<f32>()
            .map_err(|e| Error::new(Cause::InvalidData, &e.to_string()))?;

        Ok((min_avg, fivemin_avg))
    }
}

impl Module for Load {
    fn name(&self) -> &'static str {
        "load"
    }

    fn output(&mut self) -> CausedResult<ModuleOutput> {
        let (one_min, five_min) = Self::get_load_avg()?;

        let is_urgent = one_min > self.alert_threshold;

        let color = if is_urgent {
            self.critical_color.clone()
        } else {
            self.ok_color.clone()
        };

        Ok(ModuleOutput::new("load")
            .with_urgent(is_urgent)
            .with_full_text(format!("{:.2} {:.2}", one_min, five_min))
            .with_color(color))
    }
}
