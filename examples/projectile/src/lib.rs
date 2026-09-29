//! Projectile ballistics formulas, documented with `formularium`.
//!
//! Each function is a textbook closed-form expression; [`formularium::formula_doc`]
//! renders it as KaTeX in the rustdoc output (`cargo doc --open`).

use formularium::formula_doc;

/// Horizontal range of a projectile launched over flat ground.
///
/// * `v0` — initial speed (m/s)
/// * `theta` — launch angle above the horizon (radians)
/// * `g` — gravitational acceleration (m/s²)
#[formula_doc]
pub fn range(v0: f64, theta: f64, g: f64) -> f64 {
    let flight_time = 2.0 * v0 * theta.sin() / g;
    v0 * theta.cos() * flight_time
}

/// Maximum height reached by the projectile.
#[formula_doc]
pub fn peak_height(v0: f64, theta: f64, g: f64) -> f64 {
    let apex_time = v0 * theta.sin() / g;
    v0 * theta.sin() * apex_time - 0.5 * g * apex_time.powi(2)
}

/// Height of the trajectory at horizontal distance `x` from the launch point.
#[formula_doc]
pub fn trajectory(x: f64, v0: f64, theta: f64, g: f64) -> f64 {
    let slope = theta.tan();
    let arc = g * x.powi(2) / (2.0 * v0.powi(2) * theta.cos().powi(2));
    x * slope - arc
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::FRAC_PI_4;

    const EPS: f64 = 1e-9;

    #[test]
    fn range_at_45_degrees() {
        // R = v0^2 * sin(2*pi/4) / g = 100 / 9.81
        assert!((range(10.0, FRAC_PI_4, 9.81) - 100.0 / 9.81).abs() < EPS);
    }

    #[test]
    fn peak_height_matches_energy_form() {
        let v0 = 10.0;
        let h = peak_height(v0, FRAC_PI_4, 9.81);
        let energy_form = (v0 * FRAC_PI_4.sin()).powi(2) / (2.0 * 9.81);
        assert!((h - energy_form).abs() < EPS);
    }

    #[test]
    fn trajectory_passes_through_apex() {
        let g = 9.81;
        let apex_x = range(10.0, FRAC_PI_4, g) / 2.0;
        assert!((trajectory(0.0, 10.0, FRAC_PI_4, g)).abs() < EPS);
        assert!(
            (trajectory(apex_x, 10.0, FRAC_PI_4, g) - peak_height(10.0, FRAC_PI_4, g)).abs() < EPS
        );
    }
}
