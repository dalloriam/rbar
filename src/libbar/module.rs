use serde::Serialize;

use rood::CausedResult;

use crate::ModuleOutput;

pub trait Module {
    fn name(&self) -> &'static str;
    fn output(&mut self) -> CausedResult<ModuleOutput>;
}
