use std::thread;
use std::time::Duration;

use rood::CausedResult;

use crate::message;
use crate::module::Module;

pub struct Bar {
    registered_modules: Vec<Box<dyn Module>>,
}

impl Bar {
    pub fn new() -> Bar {
        Bar {
            registered_modules: Vec::new(),
        }
    }

    pub fn register(&mut self, bar_mod: Box<dyn Module>) {
        self.registered_modules.push(bar_mod);
    }

    pub fn run(&mut self) {
        let version = message::Version::default();
        match serde_json::to_string(&version) {
            Ok(s) => println!("{}", s),
            Err(e) => panic!(e.to_string()),
        }
        println!("[");

        loop {
            let out_maybe: CausedResult<Vec<message::ModuleOutput>> = self
                .registered_modules
                .iter_mut()
                .rev()
                .map(|m| m.output())
                .collect();

            match out_maybe {
                Ok(outputs) => match serde_json::to_string(&outputs) {
                    Ok(s) => println!("{}", s),
                    Err(e) => panic!(e.to_string()),
                },
                Err(e) => panic!(e.to_string()),
            }

            thread::sleep(Duration::from_secs(1));
            println!(",");
        }
    }
}
