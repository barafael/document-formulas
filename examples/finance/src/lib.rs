//! Compound-interest and amortization formulas, documented with `formularium`.
//!
//! [`formularium::formula_doc`] renders each function's arithmetic as KaTeX in
//! the rustdoc output (`cargo doc --open`).

use formularium::formula_doc;

/// Future value of a lump sum under compound interest.
///
/// * `present` — principal invested today
/// * `rate` — periodic interest rate (e.g. `0.05` for 5 %)
/// * `periods` — number of compounding periods
#[formula_doc]
pub fn future_value(present: f64, rate: f64, periods: u32) -> f64 {
    present * (1.0 + rate).powf(periods as f64)
}

/// Years for money to double at a given annual rate (rule of 72).
#[formula_doc]
pub fn doubling_time(annual_rate: f64) -> f64 {
    72.0 / annual_rate
}

/// Fixed monthly payment of a fully amortizing loan.
///
/// * `principal` — amount borrowed
/// * `annual_rate` — nominal annual interest rate (e.g. `0.05`)
/// * `months` — loan term in months
#[formula_doc]
pub fn monthly_payment(principal: f64, annual_rate: f64, months: u32) -> f64 {
    let monthly_rate = annual_rate / 12.0;
    let growth = (1.0 + monthly_rate).powf(months as f64);
    principal * monthly_rate * growth / (growth - 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn future_value_compounds() {
        let fv = future_value(1000.0, 0.05, 10);
        assert!((fv - 1_628.894_626_777_442).abs() < 1e-9);
    }

    #[test]
    fn rule_of_72() {
        assert!((doubling_time(0.06) - 1200.0).abs() < 1e-9);
    }

    #[test]
    fn amortizing_payment() {
        let pmt = monthly_payment(100_000.0, 0.05, 360);
        assert!((pmt - 536.821_56).abs() < 0.001);
    }
}
