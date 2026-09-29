//! Procedurally generated math documentation, rendered with KaTeX.
//!
//! Attach [`formula_doc`] to a function and every formula-shaped expression
//! in its body (bindings with arithmetic right-hand sides, trailing and
//! `return` expressions) is lifted into the doc comment as a TeX formula,
//! rendered in rustdoc by an auto-injected KaTeX loader.
//!
//! ```
//! assert_eq!(formularium::hypotenuse(3.0, 4.0), 5.0);
//! ```

pub use formularium_macros::formula_doc;

/// Returns the sum of two unsigned integers.
#[formula_doc]
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// Computes the hypotenuse of a right-angled triangle.
///
/// # Examples
///
/// ```
/// assert_eq!(formularium::hypotenuse(3.0, 4.0), 5.0);
/// ```
#[formula_doc]
pub fn hypotenuse(a: f64, b: f64) -> f64 {
    let a2 = a.powi(2);
    let b2 = b.powi(2);
    let sum = a2 + b2;
    sum.sqrt()
}

/// Solves `a*x^2 + b*x + c = 0` for real roots, if any.
#[formula_doc]
pub fn quadratic_roots(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let disc = b * b - 4.0 * a * c;
    if disc < 0.0 {
        return None;
    }
    let root = disc.sqrt();
    let denom = 2.0 * a;
    Some(((-b + root) / denom, (-b - root) / denom))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        assert_eq!(hypotenuse(3.0, 4.0), 5.0);
        assert!(quadratic_roots(1.0, 0.0, -4.0).is_some());
        assert!(quadratic_roots(1.0, 0.0, 4.0).is_none());
    }
}
