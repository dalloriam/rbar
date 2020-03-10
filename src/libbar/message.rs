use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Version {
    version: u32,
    click_events: bool,
}

impl Default for Version {
    fn default() -> Version {
        Version {
            version: 1,
            click_events: true,
        }
    }
}

#[derive(Serialize)]
pub struct ModuleOutput {
    pub full_text: String,
    pub name: String,
    pub urgent: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

impl ModuleOutput {
    pub fn new(name: &str) -> ModuleOutput {
        ModuleOutput {
            name: String::from(name),
            full_text: String::new(),
            urgent: false,
            color: None,
        }
    }

    pub fn with_full_text(mut self, full_text: String) -> Self {
        self.full_text = full_text;
        self
    }

    pub fn with_urgent(mut self, urgent: bool) -> Self {
        self.urgent = urgent;
        self
    }

    pub fn with_color(mut self, color: String) -> Self {
        self.color = Some(color);
        self
    }
}
