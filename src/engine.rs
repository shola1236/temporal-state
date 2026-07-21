use crate::math::clamp;
use crate::state::TemporalState;

impl TemporalState {

    pub fn update(&mut self, value: f64, dt: f64) {

        if dt <= 0.0 {
            return;
        }

        let old_velocity = self.velocity;

        self.velocity = (value - self.value) / dt;

        self.acceleration =
            (self.velocity - old_velocity) / dt;

        self.value = value;

        self.prediction = self.predict(dt);

        self.uncertainty += dt.sqrt() * 0.01;

        self.confidence =
            clamp(
                1.0 - self.uncertainty,
                0.0,
                1.0,
            );

        self.samples += 1;
    }

    pub fn health(&self) -> f64 {

        self.confidence * 100.0

    }

}