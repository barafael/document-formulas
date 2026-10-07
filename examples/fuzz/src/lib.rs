#![allow(non_snake_case)]
#![allow(clippy::all)]

//! Fuzz corpus for [`document_formulas::formula_doc`].
//!
//! Every function is a deliberate input case: operators and precedence,
//! each supported method, method chains, free-function calls, identifier
//! prettification, literals, field and index access, binding shapes, and
//! realistic formulas — plus cases that must be skipped entirely (either
//! not formula-shaped, or formula-shaped but unrenderable and therefore
//! deliberately not lifted).
//!
//! Functions whose name starts with `skip_` must yield no formulas at all;
//! this is asserted by `cargo test -p document_formulas fuzz_report`, which
//! also regenerates `target/fuzz-report.md` with one entry per extracted
//! formula. The human judgment of every entry lives in this crate's
//! `JUDGEMENT.md`.

use document_formulas::formula_doc;

pub mod operators {
    use super::formula_doc;

    #[formula_doc]
    pub fn add(a: f64, b: f64) -> f64 {
        a + b
    }

    #[formula_doc]
    pub fn sub(a: f64, b: f64) -> f64 {
        a - b
    }

    #[formula_doc]
    pub fn mul(a: f64, b: f64) -> f64 {
        a * b
    }

    #[formula_doc]
    pub fn div(a: f64, b: f64) -> f64 {
        a / b
    }

    #[formula_doc]
    pub fn rem(a: f64, b: f64) -> f64 {
        a % b
    }

    #[formula_doc]
    pub fn skip_neg(a: f64) -> f64 {
        -a
    }

    #[formula_doc]
    pub fn neg_literal(a: f64) -> f64 {
        -1.0 * a
    }

    #[formula_doc]
    pub fn subtract_chain(a: f64, b: f64, c: f64) -> f64 {
        a - b - c
    }

    #[formula_doc]
    pub fn divide_chain(a: f64, b: f64, c: f64) -> f64 {
        a / b / c
    }

    #[formula_doc]
    pub fn precedence_mul_add(a: f64, b: f64, c: f64) -> f64 {
        a + b * c
    }

    #[formula_doc]
    pub fn parens_left(a: f64, b: f64, c: f64) -> f64 {
        (a + b) * c
    }

    #[formula_doc]
    pub fn parens_sub(a: f64, b: f64, c: f64) -> f64 {
        a - (b - c)
    }

    #[formula_doc]
    pub fn nested_parens(a: f64, b: f64, c: f64) -> f64 {
        (a + b) * c
    }

    #[formula_doc]
    pub fn redundant_parens(a: f64, b: f64) -> f64 {
        (a) + b
    }

    #[formula_doc]
    pub fn minus_minus(a: f64, b: f64) -> f64 {
        a - -b
    }

    #[formula_doc]
    pub fn unary_right_of_mul(a: f64, b: f64) -> f64 {
        a * -b
    }

    #[formula_doc]
    pub fn all_operators(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) -> f64 {
        a * b + c / d - e % f
    }

    #[formula_doc]
    pub fn cast_divide(n: u32) -> f64 {
        n as f64 / 2.0
    }

    #[formula_doc]
    pub fn eq_composite(a: f64, b: f64, c: f64, d: f64) -> bool {
        a * b == c + d
    }

    #[formula_doc]
    pub fn ne_composite(a: f64, b: f64) -> bool {
        a * 2.0 != b
    }

    #[formula_doc]
    pub fn lt_gt(a: f64, b: f64) -> bool {
        a * 2.0 < b + 1.0
    }

    #[formula_doc]
    pub fn le_ge(a: f64, b: f64) -> (bool, bool) {
        let s = a + b;
        (s <= b + 1.0, s >= a - 1.0)
    }

    #[formula_doc]
    pub fn rem_zero(a: f64) -> bool {
        a % 2.0 == 0.0
    }

    #[formula_doc]
    pub fn pair(a: f64, b: f64) -> (f64, f64) {
        (a + b, a - b)
    }

    #[formula_doc]
    pub fn skip_eq(a: f64, b: f64) -> bool {
        a == b
    }

    #[formula_doc]
    pub fn skip_lt(a: f64, b: f64) -> bool {
        a < b
    }

    #[formula_doc]
    pub fn const_shadowing(PI: f64) -> f64 {
        PI * 2.0
    }
}

pub mod methods {
    use super::formula_doc;

    #[formula_doc]
    pub fn sqrt(x: f64) -> f64 {
        x.sqrt()
    }

    #[formula_doc]
    pub fn cbrt(x: f64) -> f64 {
        x.cbrt()
    }

    #[formula_doc]
    pub fn abs(x: f64) -> f64 {
        x.abs()
    }

    #[formula_doc]
    pub fn recip(x: f64) -> f64 {
        x.recip()
    }

    #[formula_doc]
    pub fn exp(x: f64) -> f64 {
        x.exp()
    }

    #[formula_doc]
    pub fn ln(x: f64) -> f64 {
        x.ln()
    }

    #[formula_doc]
    pub fn log(x: f64) -> f64 {
        x.log(10.0)
    }

    #[formula_doc]
    pub fn log2(x: f64) -> f64 {
        x.log2()
    }

    #[formula_doc]
    pub fn log10(x: f64) -> f64 {
        x.log10()
    }

    #[formula_doc]
    pub fn sin(x: f64) -> f64 {
        x.sin()
    }

    #[formula_doc]
    pub fn cos(x: f64) -> f64 {
        x.cos()
    }

    #[formula_doc]
    pub fn tan(x: f64) -> f64 {
        x.tan()
    }

    #[formula_doc]
    pub fn asin(x: f64) -> f64 {
        x.asin()
    }

    #[formula_doc]
    pub fn acos(x: f64) -> f64 {
        x.acos()
    }

    #[formula_doc]
    pub fn atan(x: f64) -> f64 {
        x.atan()
    }

    #[formula_doc]
    pub fn sinh(x: f64) -> f64 {
        x.sinh()
    }

    #[formula_doc]
    pub fn cosh(x: f64) -> f64 {
        x.cosh()
    }

    #[formula_doc]
    pub fn tanh(x: f64) -> f64 {
        x.tanh()
    }

    #[formula_doc]
    pub fn asinh(x: f64) -> f64 {
        x.asinh()
    }

    #[formula_doc]
    pub fn acosh(x: f64) -> f64 {
        x.acosh()
    }

    #[formula_doc]
    pub fn atanh(x: f64) -> f64 {
        x.atanh()
    }

    #[formula_doc]
    pub fn floor(x: f64) -> f64 {
        x.floor()
    }

    #[formula_doc]
    pub fn ceil(x: f64) -> f64 {
        x.ceil()
    }

    #[formula_doc]
    pub fn round(x: f64) -> f64 {
        x.round()
    }

    #[formula_doc]
    pub fn powi(x: f64) -> f64 {
        x.powi(3)
    }

    #[formula_doc]
    pub fn powf(x: f64) -> f64 {
        x.powf(0.5)
    }

    #[formula_doc]
    pub fn pow(x: i32) -> i32 {
        x.pow(2)
    }

    #[formula_doc]
    pub fn atan2(y: f64, x: f64) -> f64 {
        y.atan2(x)
    }

    #[formula_doc]
    pub fn hypot(a: f64, b: f64) -> f64 {
        a.hypot(b)
    }

    #[formula_doc]
    pub fn min(a: f64, b: f64) -> f64 {
        a.min(b)
    }

    #[formula_doc]
    pub fn max(a: f64, b: f64) -> f64 {
        a.max(b)
    }

    #[formula_doc]
    pub fn min_composite(a: f64, b: f64, c: f64) -> f64 {
        a.min(b - c)
    }

    #[formula_doc]
    pub fn negative_exponent(a: f64) -> f64 {
        a.powi(-2)
    }

    #[formula_doc]
    pub fn negative_composite_exponent(a: f64, i: i32) -> f64 {
        a.powi(-(i + 1))
    }

    #[formula_doc]
    pub fn log_base_composite(x: f64, b: f64) -> f64 {
        x.log(b + 1.0)
    }
}

pub mod method_chains {
    use super::formula_doc;

    #[formula_doc]
    pub fn sqrt_of_sum(a: f64, b: f64) -> f64 {
        (a + b).sqrt()
    }

    #[formula_doc]
    pub fn abs_of_negative(a: f64) -> f64 {
        (-a).abs()
    }

    #[formula_doc]
    pub fn pow_chain(a: f64) -> f64 {
        a.powi(2).powi(3)
    }

    #[formula_doc]
    pub fn exp_then_ln(a: f64) -> f64 {
        a.exp().ln()
    }

    #[formula_doc]
    pub fn sin_then_cos(a: f64) -> f64 {
        a.sin().cos()
    }

    #[formula_doc]
    pub fn abs_squared(a: f64) -> f64 {
        a.abs().powi(2)
    }

    #[formula_doc]
    pub fn sqrt_of_square(a: f64) -> f64 {
        a.powi(2).sqrt()
    }

    #[formula_doc]
    pub fn min_max_chain(a: f64, b: f64, c: f64) -> f64 {
        a.max(b).min(c)
    }

    #[formula_doc]
    pub fn min_of_abs(a: f64) -> f64 {
        a.abs().min(1.0)
    }

    #[formula_doc]
    pub fn atan2_composite(y: f64, x: f64) -> f64 {
        y.atan2(x + 1.0)
    }

    #[formula_doc]
    pub fn hypot_composite(a: f64, b: f64, c: f64) -> f64 {
        a.hypot(b + c)
    }

    #[formula_doc]
    pub fn hypot_nested(a: f64, b: f64, c: f64) -> f64 {
        a.hypot(b.hypot(c))
    }

    #[formula_doc]
    pub fn recip_of_sum(a: f64, b: f64) -> f64 {
        (a + b).recip()
    }

    #[formula_doc]
    pub fn exp_of_negative(b: f64) -> f64 {
        (-b).exp()
    }

    #[formula_doc]
    pub fn exp_of_sum(a: f64, b: f64) -> f64 {
        (a + b).exp()
    }

    #[formula_doc]
    pub fn ln_of_sum(a: f64, b: f64) -> f64 {
        (a + b).ln()
    }

    #[formula_doc]
    pub fn floor_of_quotient(a: f64, b: f64) -> f64 {
        (a / b).floor()
    }

    #[formula_doc]
    pub fn sqrt_of_abs(a: f64) -> f64 {
        a.abs().sqrt()
    }

    #[formula_doc]
    pub fn neg_power(a: f64) -> f64 {
        -a.powi(2)
    }

    #[formula_doc]
    pub fn fractional_exponent(a: f64) -> f64 {
        a.powf(1.0 / 3.0)
    }

    #[formula_doc]
    pub fn composite_base(a: f64, b: f64) -> f64 {
        (a * b).powi(2)
    }

    #[formula_doc]
    pub fn composite_exponent(a: f64, b: f64, c: f64) -> f64 {
        a.powf(b + c)
    }

    #[formula_doc]
    pub fn sqrt_of_call(a: f64) -> f64 {
        sqrt(a).powi(2)
    }

    #[formula_doc]
    pub fn deep_chain(a: f64) -> f64 {
        a.abs().sqrt().exp().ln().sqrt()
    }

    #[formula_doc]
    pub fn sqrt_of_mul(a: f64, b: f64, c: f64, d: f64) -> f64 {
        ((a + b) * (c - d)).sqrt()
    }

    #[formula_doc]
    pub fn pow_of_frac(a: f64, b: f64) -> f64 {
        (a / b).powi(2)
    }

    pub fn sqrt(x: f64) -> f64 {
        x.sqrt()
    }
}

pub mod free_functions {
    use super::formula_doc;

    #[formula_doc]
    pub fn sqrt_of_expr(a: f64, b: f64) -> f64 {
        sqrt(a + b)
    }

    #[formula_doc]
    pub fn pow_of_expr(a: f64, b: f64) -> f64 {
        pow(a + b, 2)
    }

    #[formula_doc]
    pub fn hypot_of_exprs(a: f64, b: f64, c: f64) -> f64 {
        hypot(a, b + c)
    }

    #[formula_doc]
    pub fn min_nested(a: f64, b: f64, c: f64) -> f64 {
        min(a, max(b, c))
    }

    #[formula_doc]
    pub fn atan2_of_exprs(y: f64, x: f64) -> f64 {
        atan2(y, x - 1.0)
    }

    #[formula_doc]
    pub fn floor_of_quotient(a: f64, b: f64) -> f64 {
        floor(a / b)
    }

    #[formula_doc]
    pub fn log_of_expr(a: f64, b: f64) -> f64 {
        log(a * b)
    }

    #[formula_doc]
    pub fn nested_calls(a: f64) -> f64 {
        exp(ln(a) + 1.0)
    }

    pub fn sqrt(x: f64) -> f64 {
        x.sqrt()
    }

    pub fn pow(x: f64, n: u32) -> f64 {
        x.powf(n as f64)
    }

    pub fn hypot(a: f64, b: f64) -> f64 {
        a.hypot(b)
    }

    pub fn min(a: f64, b: f64) -> f64 {
        a.min(b)
    }

    pub fn max(a: f64, b: f64) -> f64 {
        a.max(b)
    }

    pub fn atan2(y: f64, x: f64) -> f64 {
        y.atan2(x)
    }

    pub fn floor(x: f64) -> f64 {
        x.floor()
    }

    pub fn log(x: f64) -> f64 {
        x.ln()
    }

    pub fn exp(x: f64) -> f64 {
        x.exp()
    }

    pub fn ln(x: f64) -> f64 {
        x.ln()
    }
}

pub mod identifiers {
    use super::formula_doc;
    use std::f64::consts::{PI, TAU};

    #[formula_doc]
    pub fn greek_single(theta: f64) -> f64 {
        theta / 2.0
    }

    #[formula_doc]
    pub fn greek_mix(alpha: f64, beta: f64, gamma: f64, omega: f64) -> f64 {
        alpha * beta + gamma - omega / 2.0
    }

    #[formula_doc]
    pub fn greek_more(mu: f64, sigma: f64, phi: f64, psi: f64, xi: f64, rho: f64) -> f64 {
        mu + sigma * phi - psi / (xi * rho)
    }

    #[formula_doc]
    pub fn greek_even_more(
        epsilon: f64,
        zeta: f64,
        eta: f64,
        iota: f64,
        kappa: f64,
        nu: f64,
        upsilon: f64,
        chi: f64,
    ) -> f64 {
        epsilon + zeta * (eta + iota) - kappa / (nu + upsilon) + chi
    }

    #[formula_doc]
    pub fn greek_digit(theta2: f64) -> f64 {
        theta2 + 1.0
    }

    #[formula_doc]
    pub fn greek_digit_two(sigma2: f64) -> f64 {
        sigma2 / 2.0
    }

    #[formula_doc]
    pub fn pi_lowercase(pi: f64) -> f64 {
        pi * 2.0
    }

    #[formula_doc]
    pub fn lambda_digit(lambda1: f64, lambda2: f64) -> f64 {
        lambda1 * lambda2
    }

    #[formula_doc]
    pub fn tau_is_not_greek(tau: f64) -> f64 {
        tau * 2.0
    }

    #[formula_doc]
    pub fn capital_is_not_greek(Eta: f64) -> f64 {
        Eta.abs()
    }

    #[formula_doc]
    pub fn subscripts(a2: f64, b2: f64, t0: f64, t1: f64) -> f64 {
        a2 + b2 - t0 * t1
    }

    #[formula_doc]
    pub fn multi_digit_suffix(x00: f64) -> f64 {
        x00 * 2.0
    }

    #[formula_doc]
    pub fn underscore_names(my_var: f64, other_thing: f64) -> f64 {
        my_var + other_thing
    }

    #[formula_doc]
    pub fn underscore_digit_suffix(a2_b: f64) -> f64 {
        a2_b.sqrt()
    }

    #[formula_doc]
    pub fn constants_pi(r: f64) -> f64 {
        PI * r * r
    }

    #[formula_doc]
    pub fn constants_tau(period: f64) -> f64 {
        TAU / period
    }

    #[formula_doc]
    pub fn constants_e(rate: f64, t: f64) -> f64 {
        (rate * t).exp()
    }

    #[formula_doc]
    pub fn qualified_constants(a: f64) -> f64 {
        std::f64::consts::PI * a + std::f64::consts::TAU
    }
}

pub mod literals {
    use super::formula_doc;

    #[formula_doc]
    pub fn ints(n: u64) -> u64 {
        n * 2 + 1
    }

    #[formula_doc]
    pub fn floats(a: f64) -> f64 {
        a * 2.5
    }

    #[formula_doc]
    pub fn underscored(a: f64) -> f64 {
        a * 1_000_000.0
    }

    #[formula_doc]
    pub fn suffix_float(a: f64) -> f64 {
        2f64 * a
    }

    #[formula_doc]
    pub fn suffix_int(a: f64) -> f64 {
        a * 3u32 as f64
    }

    #[formula_doc]
    pub fn hex(a: f64) -> f64 {
        0x10 as f64 * a
    }

    #[formula_doc]
    pub fn scientific(a: f64) -> f64 {
        1.5e-8 * a
    }

    #[formula_doc]
    pub fn negative_literal(a: f64) -> f64 {
        -2.0 * a
    }

    #[formula_doc]
    pub fn int_division(n: u64) -> u64 {
        1000000 / n
    }

    #[formula_doc]
    pub fn sci_mantissa(a: f64) -> f64 {
        1e3 * a
    }

    #[formula_doc]
    pub fn sci_in_exponent(a: f64) -> f64 {
        a.powf(1e-3)
    }

    #[formula_doc]
    pub fn binary_literal(a: f64) -> f64 {
        0b1010 as f64 * a
    }

    #[formula_doc]
    pub fn octal_literal(a: f64) -> f64 {
        0o17 as f64 * a
    }
}

pub mod access {
    use super::formula_doc;

    pub struct Rect {
        pub width: f64,
        pub height: f64,
    }

    pub struct Nested {
        pub inner: Rect,
        pub gain: f64,
    }

    #[formula_doc]
    pub fn fields(r: &Rect) -> f64 {
        r.width * r.height
    }

    #[formula_doc]
    pub fn field_chain(n: &Nested) -> f64 {
        n.inner.width + n.gain
    }

    #[formula_doc]
    pub fn method_on_field(r: &Rect) -> f64 {
        r.width.sqrt() * r.height.abs()
    }

    #[formula_doc]
    pub fn index_const(xs: [f64; 2]) -> f64 {
        xs[0] + xs[1]
    }

    #[formula_doc]
    pub fn index_expr(xs: [f64; 3], i: usize) -> f64 {
        xs[i] * xs[i + 1]
    }

    #[formula_doc]
    pub fn tuple_fields(t: (f64, f64)) -> f64 {
        t.0 + t.1
    }
}

pub mod binders {
    use super::formula_doc;

    #[formula_doc]
    pub fn quadratic(a: f64, b: f64, c: f64) -> f64 {
        let two_a = 2.0 * a;
        let disc = b * b - 4.0 * a * c;
        let root = (-b + disc.sqrt()) / two_a;
        root
    }

    #[formula_doc]
    pub fn shadowing(x: f64) -> f64 {
        let y = x + 1.0;
        let y = y * 2.0;
        y
    }

    #[formula_doc]
    pub fn let_else(opt: Option<f64>) -> f64 {
        let Some(v) = opt else { return 0.0 };
        v * 2.0
    }

    #[formula_doc]
    pub fn if_else_pick(a: f64, b: f64) -> f64 {
        let m = if a > b { a } else { b };
        m * 2.0
    }

    #[formula_doc]
    pub fn block_binding(a: f64) -> f64 {
        let x = { a + 1.0 };
        x * 2.0
    }

    #[formula_doc]
    pub fn explicit_return(a: f64, b: f64) -> f64 {
        return a * b;
    }

    #[formula_doc]
    #[allow(unreachable_code)]
    pub fn diverging_init(a: f64) -> f64 {
        let x = a * 2.0;
        let boom = x.min(loop {});
        boom
    }

    #[formula_doc]
    pub fn skip_loop_accumulate(a: f64) -> f64 {
        let mut acc = 0.0;
        for _ in 0..3 {
            acc += a;
        }
        acc
    }

    #[formula_doc]
    pub fn skip_closure(a: f64) -> f64 {
        let f = |x: f64| x + 1.0;
        f(a)
    }

    #[formula_doc]
    pub fn vec_macro_binding(a: f64, b: f64) -> f64 {
        let v = vec![a, b];
        v[0] + v[1]
    }

    #[formula_doc]
    pub fn skip_return_nonmath(s: &str) -> usize {
        return s.len();
    }
}

pub mod realistic {
    use super::formula_doc;
    use std::f64::consts::TAU;

    #[formula_doc]
    pub fn hypotenuse(a: f64, b: f64) -> f64 {
        let a2 = a.powi(2);
        let b2 = b.powi(2);
        let sum = a2 + b2;
        sum.sqrt()
    }

    #[formula_doc]
    pub fn projectile_range(v0: f64, theta: f64, g: f64) -> f64 {
        v0.powi(2) * (2.0 * theta).sin() / g
    }

    #[formula_doc]
    pub fn kinetic_energy(m: f64, v: f64) -> f64 {
        0.5 * m * v.powi(2)
    }

    #[formula_doc]
    pub fn stefan_boltzmann(sigma: f64, area: f64, t: f64) -> f64 {
        sigma * area * t.powi(4)
    }

    #[formula_doc]
    pub fn logistic(x: f64) -> f64 {
        1.0 / (1.0 + (-x).exp())
    }

    #[formula_doc]
    pub fn normal_pdf(x: f64, mu: f64, sigma: f64) -> f64 {
        let z = (x - mu) / sigma;
        let num = (-z.powi(2) / 2.0).exp();
        num / (sigma * TAU.sqrt())
    }

    #[formula_doc]
    pub fn lorentz_factor(v: f64, c: f64) -> f64 {
        1.0 / (1.0 - (v / c).powi(2)).sqrt()
    }

    #[formula_doc]
    pub fn shannon_bits(p: f64) -> f64 {
        -p * p.log2() - (1.0 - p) * (1.0 - p).log2()
    }

    #[formula_doc]
    pub fn compound_growth(principal: f64, rate: f64, periods: u32) -> f64 {
        let growth = (1.0 + rate).powf(periods as f64);
        principal * growth
    }

    #[formula_doc]
    pub fn root_mean_square(a: f64, b: f64) -> f64 {
        ((a.powi(2) + b.powi(2)) / 2.0).sqrt()
    }

    #[formula_doc]
    pub fn escape_velocity(mass: f64, radius: f64) -> f64 {
        (2.0 * 6.674e-11 * mass / radius).sqrt()
    }
}

pub mod skips {
    use super::formula_doc;

    #[formula_doc]
    pub fn skip_strings(s: &str) -> usize {
        let shout = format!("{}!", s);
        shout.len()
    }
    #[formula_doc]
    pub fn skip_control_flow(a: f64, b: f64) -> f64 {
        if a > b { a } else { b }
    }

    #[formula_doc]
    pub fn skip_bool_logic(a: f64, b: f64) -> bool {
        a > 0.0 && b > 0.0
    }

    #[formula_doc]
    pub fn skip_assignment(a: f64) -> f64 {
        let mut acc = a;
        acc = acc * 2.0;
        acc
    }

    #[formula_doc]
    pub fn skip_match(a: f64) -> f64 {
        match a {
            v => v * 1.0,
        }
    }

    #[formula_doc]
    pub fn skip_empty() {}
}

pub mod weird {
    use super::formula_doc;
    use std::f64::consts::PI;

    #[formula_doc]
    pub fn qualified_calls(a: f64, b: f64) -> f64 {
        f64::powi(a, 2) + f64::hypot(a, b)
    }

    #[formula_doc]
    pub fn deeply_nested(a: f64, b: f64, c: f64, d: f64) -> f64 {
        ((a + b) * (c - d)).sqrt().abs().powi(2)
    }

    #[formula_doc]
    pub fn rem_in_hypot(a: f64, x: f64, y: f64) -> f64 {
        a.hypot(x % y)
    }

    #[formula_doc]
    pub fn mega_chain(a: f64, b: f64, c: f64, d: f64, f: f64, g: f64, h: f64) -> f64 {
        let q = a * b + c / d;
        let r = q.hypot(f) - g.sqrt();
        r.powf(h + a) * PI
    }

    #[formula_doc]
    pub fn log_base(x: f64) -> f64 {
        x.log(2.0)
    }

    #[formula_doc]
    pub fn nonmath_method_arithmetic(s: &str) -> usize {
        s.len() + s.chars().count()
    }

    #[formula_doc]
    pub fn turbofish_binding(s: &str) -> f64 {
        let n = "3.5".parse::<f64>().unwrap_or(0.0);
        n + s.len() as f64
    }
}
