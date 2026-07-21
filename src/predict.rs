use crate::state::TemporalState;

impl TemporalState {
    pub fn predict(&self, dt: f64) -> f64 {
        self.value
            + self.velocity * dt
            + 0.5 * self.acceleration * dt * dt
    }
}