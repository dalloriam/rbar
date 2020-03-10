use std::time::SystemTime;

use chrono::offset::Local;
use chrono::DateTime;

use libbar::{Module, ModuleOutput};

pub struct Clock {}

impl Clock {
    pub fn new() -> Clock {
        Clock {}
    }

    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

impl Module for Clock {
    fn name(&self) -> &'static str {
        "clock"
    }

    fn output(&mut self) -> rood::CausedResult<ModuleOutput> {
        let cur_time = SystemTime::now();

        let datetime: DateTime<Local> = cur_time.into();
        Ok(ModuleOutput::new("clock").with_full_text(datetime.format("%a %-d %b %X").to_string()))
    }
}
