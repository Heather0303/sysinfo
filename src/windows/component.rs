// 
// Sysinfo
// 
// Copyright (c) 2018 Guillaume Gomez
//

use ComponentExt;

/// Struct containing a component information (temperature and name for the moment).
pub struct Component {
    /// Temperature is in celsius.
    pub temperature: f32,
    /// Temperature max value.
    pub max: f32,
    /// The highest temperature before the computer halts.
    pub critical: Option<f32>,
    /// Component's label.
    pub label: String,
}

impl Component {
    /// Creates a new `Component` with the given information.
    pub fn new(label: String, max: Option<f32>, critical: Option<f32>) -> Component {
        Component {
            temperature: 0f32,
            label: label,
            max: max.unwrap_or(0.0),
            critical: critical,
        }
    }
}

impl ComponentExt for Component {
    fn get_temperature(&self) -> f32 {
        self.temperature
    }

    fn get_max(&self) -> f32 {
        self.max
    }

    fn get_critical(&self) -> Option<f32> {
        self.critical
    }

    fn get_label(&self) -> &str {
        &self.label
    }
}

pub fn get_components() -> Vec<Component> {
    Vec::new()
}
