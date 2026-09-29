//! A scalar (1D) Kalman filter, documented with `document_formulas`.
//!
//! Method bodies are written so the arithmetic mirrors the classic filter
//! equations one to one; [`document_formulas::formula_doc`] lifts them into the
//! rustdoc output, where KaTeX renders them. Run `cargo doc --open` and
//! visit the `predict` and `update` methods.

use document_formulas::formula_doc;

/// A scalar Kalman filter estimating a single noisy signal.
#[derive(Debug, Clone)]
pub struct Kalman1D {
    state: f64,
    covariance: f64,
    process_noise: f64,
    measurement_noise: f64,
}

impl Kalman1D {
    /// Creates a filter with an initial guess and its uncertainty.
    pub fn new(
        initial_state: f64,
        initial_covariance: f64,
        process_noise: f64,
        measurement_noise: f64,
    ) -> Self {
        Self {
            state: initial_state,
            covariance: initial_covariance,
            process_noise,
            measurement_noise,
        }
    }

    /// Time update: advances the state with a constant-acceleration motion
    /// model and inflates the uncertainty by the process noise.
    #[formula_doc]
    pub fn predict(&mut self, acceleration: f64, dt: f64) {
        let transitioned = self.state + 0.5 * acceleration * dt.powi(2);
        let propagated = self.covariance + self.process_noise;
        self.state = transitioned;
        self.covariance = propagated;
    }

    /// Measurement update: fuses the prediction with a new measurement,
    /// weighting it against the prediction by the Kalman gain.
    #[formula_doc]
    pub fn update(&mut self, measurement: f64) {
        let innovation = measurement - self.state;
        let gain = self.covariance / (self.covariance + self.measurement_noise);
        let corrected = self.state + gain * innovation;
        let covariance = (1.0 - gain) * self.covariance;
        self.state = corrected;
        self.covariance = covariance;
    }

    /// Current state estimate.
    pub fn state(&self) -> f64 {
        self.state
    }

    /// Current estimate covariance.
    pub fn covariance(&self) -> f64 {
        self.covariance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn predict_integrates_acceleration() {
        let mut filter = Kalman1D::new(0.0, 1.0, 0.01, 1.0);
        filter.predict(2.0, 1.0);
        assert!((filter.state() - 1.0).abs() < 1e-9);
        assert!((filter.covariance() - 1.01).abs() < 1e-9);
    }

    #[test]
    fn update_pulls_estimate_toward_measurement() {
        let mut filter = Kalman1D::new(0.0, 1.0, 0.0, 1.0);
        filter.update(10.0);
        let gain = 1.0 / 2.0;
        assert!((filter.state() - gain * 10.0).abs() < 1e-9);
        assert!((filter.covariance() - gain).abs() < 1e-9);
    }

    #[test]
    fn converges_to_constant_signal() {
        let mut filter = Kalman1D::new(0.0, 100.0, 0.01, 1.0);
        for _ in 0..100 {
            filter.predict(0.0, 1.0);
            filter.update(10.0);
        }
        assert!((filter.state() - 10.0).abs() < 0.01);
        assert!(filter.covariance() < 0.2);
    }
}
