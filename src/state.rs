#[derive(Debug, Clone)]
pub struct TemporalState {
    pub value: f64,
    pub velocity: f64,
    pub acceleration: f64,

    pub uncertainty: f64,
    pub confidence: f64,

    pub prediction: f64,

    pub last_time: f64,

    pub samples: u64,
}

impl Default for TemporalState {
    fn default() -> Self {
        Self {
            value: 0.0,
            velocity: 0.0,
            acceleration: 0.0,

            uncertainty: 0.0,
            confidence: 1.0,

            prediction: 0.0,

            last_time: 0.0,

            samples: 0,
        }
    }
}