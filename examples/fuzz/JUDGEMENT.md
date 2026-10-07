# Fuzz judgement

Per-formula verdicts for the [`fuzz`](src/lib.rs) corpus, judged against the
auto-generated `target/fuzz-report.md` (regenerate with
`cargo test -p document_formulas fuzz_report`).

Corpus totals: 180 formula-shaped candidates, 177 rendered and extracted,
3 deliberately not extracted (formula-shaped but unrenderable), 12 `skip_*`
functions correctly producing nothing. The harness asserts: extraction is
exactly the renderable candidates, and the only unrenderable candidates are
the three pinned below.

Verdict key:

- **match** — the TeX renders exactly the arithmetic the Rust expresses.
- **match\*** — semantically faithful, with a cosmetic or minor note.
- **skip** — correctly not lifted at all.
- **not extracted** — formula-shaped, but deliberately not lifted because
  it cannot be rendered.

## Round 1 findings and their resolutions

1. **`x.log(base)` was silently dropped.** Fixed: `x.log(b)` now renders as
   `\log_{b}\left(x\right)` (`methods::log`, `weird::log_base`,
   `methods::log_base_composite`).
2. **Scientific notation rendered verbatim** (`1.5e-8` reading as
   “1.5e − 8”). Fixed: `1.5e-8` → `1.5 \times 10^{-8}`, also inside
   exponent slots (`literals::sci_in_exponent`, `realistic::escape_velocity`).
3. **Hex literals rendered in math italics.** Fixed per decision: verbatim
   but upright — `0x10` → `\mathtt{0x10}` (same for `0o17`, `0b1010`).
4. **Greek prettification didn't compose with subscripts, and lowercase
   `pi`/`tau` were unmapped.** Fixed: `theta2` → `\theta_{2}`,
   `lambda1` → `\lambda_{1}`, `pi` → `\pi`, `tau` → `\tau`.
5. **Underscored names block subscripts** (`a2_b` → `a2\_b`). Kept by
   decision: underscored identifiers render escaped only.
6. **Extracted-but-unrenderable formulas silently vanished.** Fixed
   structurally: extraction now requires renderability, so the docs never
   advertise a line that then disappears. Three formula-shaped constructs
   remain unextractable and are pinned by assertion: `x.min(loop {})`
   (`binders::diverging_init`), `s.len() + s.chars().count()`
   (`weird::nonmath_method_arithmetic`), `n + s.len() as f64`
   (`weird::turbofish_binding`).
7. **Double parentheses** around parenthesised receivers
   (`\ln\left((a + b)\right)`, `e^{(-x)}`, `\sqrt{(a + b)}`,
   `\frac{1}{(a + b)}`). Fixed: redundant groups are stripped in every
   delimiting slot.
8. **Exponents attached to `\frac`/`\sqrt` bases without braces.** Fixed:
   `(a / b).powi(2)` → `{\frac{a}{b}}^{2}`, `sqrt(a).powi(2)` →
   `{\sqrt{a}}^{2}`, and chained `powi` renders explicitly as
   `\left(a^{2}\right)^{3}`.
9. **Tuple fields were unrenderable.** Fixed per decision: `t.0 + t.1` →
   `t_{0} + t_{1}`.
10. **Integer division renders as an exact fraction** — remains: the macro
    has no type information, so `1000000 / n` (u64) shows `\frac{1000000}{n}`
    without the truncation caveat. Documented, accepted.
11. **Casts are silently dropped** (`n as f64` → `n`) — remains by design.

## operators

| function | input | generated TeX | verdict |
|---|---|---|---|
| add | `a + b` | `a + b` | match |
| sub | `a - b` | `a - b` | match |
| mul | `a * b` | `a \cdot b` | match |
| div | `a / b` | `\frac{a}{b}` | match |
| rem | `a % b` | `a \bmod b` | match |
| skip_neg | `-a` | *(none)* | skip (no arithmetic operator) |
| neg_literal | `-1.0 * a` | `-1.0 \cdot a` | match |
| subtract_chain | `a - b - c` | `a - b - c` | match (left-assoc preserved) |
| divide_chain | `a / b / c` | `\frac{\frac{a}{b}}{c}` | match |
| precedence_mul_add | `a + b * c` | `a + b \cdot c` | match |
| parens_left | `(a + b) * c` | `(a + b) \cdot c` | match |
| parens_sub | `a - (b - c)` | `a - (b - c)` | match (significant parens kept) |
| nested_parens | `((a + b)) * c` | `(a + b) \cdot c` | match (redundant parens dropped) |
| redundant_parens | `(a) + b` | `a + b` | match |
| minus_minus | `a - -b` | `a - -b` | match\* (terse; `a - (-b)` nicer) |
| unary_right_of_mul | `a * -b` | `a \cdot -b` | match\* |
| all_operators | `a * b + c / d - e % f` | `a \cdot b + \frac{c}{d} - e \bmod f` | match |
| cast_divide | `n as f64 / 2.0` | `\frac{n}{2.0}` | match (cast dropped, finding 11) |
| eq_composite | `a * b == c + d` | `a \cdot b = c + d` | match |
| ne_composite | `a * 2.0 != b` | `a \cdot 2.0 \neq b` | match |
| lt_gt | `a * 2.0 < b + 1.0` | `a \cdot 2.0 < b + 1.0` | match |
| le_ge | `let s = a + b; (s <= b + 1.0, s >= a - 1.0)` | `s = a + b`; `\left(s \le b + 1.0, s \ge a - 1.0\right)` | match |
| rem_zero | `a % 2.0 == 0.0` | `a \bmod 2.0 = 0.0` | match |
| pair | `(a + b, a - b)` | `\left(a + b, a - b\right)` | match |
| skip_eq | `a == b` | *(none)* | skip (bare comparison, by design) |
| skip_lt | `a < b` | *(none)* | skip (bare comparison, by design) |
| const_shadowing | `PI * 2.0` (param named `PI`) | `\pi \cdot 2.0` | match\* (variable rendered as constant glyph) |

## methods

| function | input | generated TeX | verdict |
|---|---|---|---|
| sqrt | `x.sqrt()` | `\sqrt{x}` | match |
| cbrt | `x.cbrt()` | `\sqrt[3]{x}` | match |
| abs | `x.abs()` | `\left\|x\right\|` | match |
| recip | `x.recip()` | `\frac{1}{x}` | match |
| exp | `x.exp()` | `e^{x}` | match |
| ln | `x.ln()` | `\ln\left(x\right)` | match |
| log | `x.log(10.0)` | `\log_{10.0}\left(x\right)` | match (fixed, finding 1) |
| log_base_composite | `x.log(b + 1.0)` | `\log_{b + 1.0}\left(x\right)` | match |
| log2 | `x.log2()` | `\log_{2}\left(x\right)` | match |
| log10 | `x.log10()` | `\log_{10}\left(x\right)` | match |
| sin/cos/tan | `x.sin()` … | `\sin\left(x\right)` … | match |
| asin/acos/atan | `x.asin()` … | `\arcsin\left(x\right)` … | match |
| sinh/cosh/tanh | `x.sinh()` … | `\sinh\left(x\right)` … | match |
| asinh/acosh/atanh | `x.asinh()` … | `\operatorname{asinh}\left(x\right)` … | match |
| floor | `x.floor()` | `\left\lfloor x\right\rfloor` | match |
| ceil | `x.ceil()` | `\left\lceil x\right\rceil` | match |
| round | `x.round()` | `\operatorname{round}\left(x\right)` | match |
| powi | `x.powi(3)` | `x^{3}` | match |
| powf | `x.powf(0.5)` | `x^{0.5}` | match |
| pow | `x.pow(2)` (i32) | `x^{2}` | match |
| atan2 | `y.atan2(x)` | `\operatorname{atan2}\left(y, x\right)` | match |
| hypot | `a.hypot(b)` | `\sqrt{a^{2} + b^{2}}` | match |
| min / max | `a.min(b)` / `a.max(b)` | `\min\left(a, b\right)` / `\max\left(a, b\right)` | match |
| min_composite | `a.min(b - c)` | `\min\left(a, b - c\right)` | match |
| negative_exponent | `a.powi(-2)` | `a^{-2}` | match |
| negative_composite_exponent | `a.powi(-(i + 1))` | `a^{-(i + 1)}` | match |

## method_chains

| function | input | generated TeX | verdict |
|---|---|---|---|
| sqrt_of_sum | `(a + b).sqrt()` | `\sqrt{a + b}` | match (fixed, finding 7) |
| abs_of_negative | `(-a).abs()` | `\left\|-a\right\|` | match (fixed) |
| pow_chain | `a.powi(2).powi(3)` | `\left(a^{2}\right)^{3}` | match (fixed, finding 8) |
| exp_then_ln | `a.exp().ln()` | `\ln\left(e^{a}\right)` | match |
| sin_then_cos | `a.sin().cos()` | `\cos\left(\sin\left(a\right)\right)` | match |
| abs_squared | `a.abs().powi(2)` | `\left\|a\right\|^{2}` | match |
| sqrt_of_square | `a.powi(2).sqrt()` | `\sqrt{a^{2}}` | match |
| min_max_chain | `a.max(b).min(c)` | `\min\left(\max\left(a, b\right), c\right)` | match |
| min_of_abs | `a.abs().min(1.0)` | `\min\left(\left\|a\right\|, 1.0\right)` | match |
| atan2_composite | `y.atan2(x + 1.0)` | `\operatorname{atan2}\left(y, x + 1.0\right)` | match |
| hypot_composite | `a.hypot(b + c)` | `\sqrt{a^{2} + (b + c)^{2}}` | match (parens correctly added) |
| hypot_nested | `a.hypot(b.hypot(c))` | `\sqrt{a^{2} + \left(\sqrt{b^{2} + c^{2}}\right)^{2}}` | match (single parens now) |
| recip_of_sum | `(a + b).recip()` | `\frac{1}{a + b}` | match (fixed) |
| exp_of_negative | `(-b).exp()` | `e^{-b}` | match (fixed) |
| exp_of_sum | `(a + b).exp()` | `e^{a + b}` | match (fixed) |
| ln_of_sum | `(a + b).ln()` | `\ln\left(a + b\right)` | match (fixed, finding 7) |
| floor_of_quotient | `(a / b).floor()` | `\left\lfloor \frac{a}{b}\right\rfloor` | match |
| sqrt_of_abs | `a.abs().sqrt()` | `\sqrt{\left\|a\right\|}` | match |
| neg_power | `-a.powi(2)` | `-a^{2}` | match (Rust parses `-(a²)`) |
| fractional_exponent | `a.powf(1.0 / 3.0)` | `a^{\frac{1.0}{3.0}}` | match |
| composite_base | `(a * b).powi(2)` | `(a \cdot b)^{2}` | match |
| composite_exponent | `a.powf(b + c)` | `a^{b + c}` | match |
| sqrt_of_call | `sqrt(a).powi(2)` | `{\sqrt{a}}^{2}` | match (fixed, finding 8) |
| deep_chain | `a.abs().sqrt().exp().ln().sqrt()` | `\sqrt{\ln\left(e^{\sqrt{\left\|a\right\|}}\right)}` | match |
| sqrt_of_mul | `((a + b) * (c - d)).sqrt()` | `\sqrt{(a + b) \cdot (c - d)}` | match (fixed, finding 7) |
| pow_of_frac | `(a / b).powi(2)` | `{\frac{a}{b}}^{2}` | match (fixed, finding 8) |
| sqrt (helper) | `x.sqrt()` | `\sqrt{x}` | match |

## free_functions

| function | input | generated TeX | verdict |
|---|---|---|---|
| sqrt_of_expr | `sqrt(a + b)` | `\sqrt{a + b}` | match |
| pow_of_expr | `pow(a + b, 2)` | `(a + b)^{2}` | match |
| hypot_of_exprs | `hypot(a, b + c)` | `\sqrt{a^{2} + (b + c)^{2}}` | match |
| min_nested | `min(a, max(b, c))` | `\min\left(a, \max\left(b, c\right)\right)` | match |
| atan2_of_exprs | `atan2(y, x - 1.0)` | `\operatorname{atan2}\left(y, x - 1.0\right)` | match |
| floor_of_quotient | `floor(a / b)` | `\left\lfloor \frac{a}{b}\right\rfloor` | match |
| log_of_expr | `log(a * b)` | `\log\left(a \cdot b\right)` | match |
| nested_calls | `exp(ln(a) + 1.0)` | `e^{\ln\left(a\right) + 1.0}` | match |
| pow (helper) | `x.powf(n as f64)` | `x^{n}` | match (cast dropped) |
| other helpers | `x.sqrt()` … | as above | match |

## identifiers

| function | input | generated TeX | verdict |
|---|---|---|---|
| greek_single | `theta / 2.0` | `\frac{\theta}{2.0}` | match |
| greek_mix | `alpha * beta + gamma - omega / 2.0` | `\alpha \cdot \beta + \gamma - \frac{\omega}{2.0}` | match |
| greek_more | `mu + sigma * phi - psi / (xi * rho)` | `\mu + \sigma \cdot \phi - \frac{\psi}{\xi \cdot \rho}` | match |
| greek_even_more | `epsilon + zeta * (eta + iota) - kappa / (nu + upsilon) + chi` | `\epsilon + \zeta \cdot (\eta + \iota) - \frac{\kappa}{\nu + \upsilon} + \chi` | match |
| greek_digit | `theta2 + 1.0` | `\theta_{2} + 1.0` | match (fixed, finding 4) |
| greek_digit_two | `sigma2 / 2.0` | `\frac{\sigma_{2}}{2.0}` | match |
| lambda_digit | `lambda1 * lambda2` | `\lambda_{1} \cdot \lambda_{2}` | match (fixed) |
| pi_lowercase | `pi * 2.0` | `\pi \cdot 2.0` | match (fixed) |
| tau_is_not_greek | `tau * 2.0` | `\tau \cdot 2.0` | match (fixed; name is historical) |
| capital_is_not_greek | `Eta.abs()` | `\left\|Eta\right\|` | match (no mapping for capitalised, expected) |
| subscripts | `a2 + b2 - t0 * t1` | `a_{2} + b_{2} - t_{0} \cdot t_{1}` | match |
| multi_digit_suffix | `x00 * 2.0` | `x_{00} \cdot 2.0` | match |
| underscore_names | `my_var + other_thing` | `my\_var + other\_thing` | match (escaped for TeX) |
| underscore_digit_suffix | `a2_b.sqrt()` | `\sqrt{a2\_b}` | match\* (finding 5, kept by decision) |
| constants_pi | `PI * r * r` | `\pi \cdot r \cdot r` | match |
| constants_tau | `TAU / period` | `\frac{\tau}{period}` | match |
| constants_e | `(rate * t).exp()` | `e^{rate \cdot t}` | match (fixed, finding 7) |
| qualified_constants | `std::f64::consts::PI * a + std::f64::consts::TAU` | `\pi \cdot a + \tau` | match |

## literals

| function | input | generated TeX | verdict |
|---|---|---|---|
| ints | `n * 2 + 1` | `n \cdot 2 + 1` | match |
| floats | `a * 2.5` | `a \cdot 2.5` | match |
| underscored | `a * 1_000_000.0` | `a \cdot 1000000.0` | match |
| suffix_float | `2f64 * a` | `2 \cdot a` | match |
| suffix_int | `a * 3u32 as f64` | `a \cdot 3` | match |
| hex | `0x10 as f64 * a` | `\mathtt{0x10} \cdot a` | match (fixed per decision, finding 3) |
| binary_literal | `0b1010 as f64 * a` | `\mathtt{0b1010} \cdot a` | match |
| octal_literal | `0o17 as f64 * a` | `\mathtt{0o17} \cdot a` | match |
| scientific | `1.5e-8 * a` | `1.5 \times 10^{-8} \cdot a` | match (fixed, finding 2) |
| sci_mantissa | `1e3 * a` | `1 \times 10^{3} \cdot a` | match |
| sci_in_exponent | `a.powf(1e-3)` | `a^{1 \times 10^{-3}}` | match |
| negative_literal | `-2.0 * a` | `-2.0 \cdot a` | match |
| int_division | `1000000 / n` (u64) | `\frac{1000000}{n}` | match\* (finding 10: hides truncation) |

## access

| function | input | generated TeX | verdict |
|---|---|---|---|
| fields | `r.width * r.height` | `r.width \cdot r.height` | match |
| field_chain | `n.inner.width + n.gain` | `n.inner.width + n.gain` | match |
| method_on_field | `r.width.sqrt() * r.height.abs()` | `\sqrt{r.width} \cdot \left\|r.height\right\|` | match |
| index_const | `xs[0] + xs[1]` | `xs[0] + xs[1]` | match\* (brackets literal, not subscript) |
| index_expr | `xs[i] * xs[i + 1]` | `xs[i] \cdot xs[i + 1]` | match\* |
| tuple_fields | `t.0 + t.1` | `t_{0} + t_{1}` | match (fixed per decision, finding 9) |

## binders

| function | input | generated TeX | verdict |
|---|---|---|---|
| quadratic | `let two_a = 2.0 * a; let disc = b * b - 4.0 * a * c; let root = (-b + disc.sqrt()) / two_a` | `two\_a = 2.0 \cdot a`; `disc = b \cdot b - 4.0 \cdot a \cdot c`; `root = \frac{-b + \sqrt{disc}}{two\_a}` | match (the quadratic formula, verbatim) |
| shadowing | `let y = x + 1.0; let y = y * 2.0` | `y = x + 1.0`; `y = y \cdot 2.0` | match |
| let_else | `let Some(v) = opt else …; v * 2.0` | `v \cdot 2.0` | match\* (let-else pattern not lifted, by design) |
| if_else_pick | `let m = if …; m * 2.0` | `m \cdot 2.0` | match\* (non-arithmetic init not lifted) |
| block_binding | `let x = { a + 1.0 }; x * 2.0` | `x \cdot 2.0` | match\* (block init not lifted) |
| explicit_return | `return a * b` | `a \cdot b` | match |
| diverging_init | `let x = a * 2.0; let boom = x.min(loop {})` | `x = a \cdot 2.0`; boom not extracted | match\* + **not extracted** (pinned, finding 6) |
| skip_loop_accumulate | `for` loop accumulation | *(none)* | skip |
| skip_closure | closure binding | *(none)* | skip |
| vec_macro_binding | `let v = vec![a, b]; v[0] + v[1]` | `v[0] + v[1]` | match\* |
| skip_return_nonmath | `return s.len()` | *(none)* | skip |

## realistic

| function | input | generated TeX | verdict |
|---|---|---|---|
| hypotenuse | `let a2 = a.powi(2); …; sum.sqrt()` | `a_{2} = a^{2}`; `b_{2} = b^{2}`; `sum = a_{2} + b_{2}`; `\sqrt{sum}` | match |
| projectile_range | `v0.powi(2) * (2.0 * theta).sin() / g` | `\frac{v_{0}^{2} \cdot \sin\left(2.0 \cdot \theta\right)}{g}` | match (fixed, finding 7) |
| kinetic_energy | `0.5 * m * v.powi(2)` | `0.5 \cdot m \cdot v^{2}` | match |
| stefan_boltzmann | `sigma * area * t.powi(4)` | `\sigma \cdot area \cdot t^{4}` | match |
| logistic | `1.0 / (1.0 + (-x).exp())` | `\frac{1.0}{1.0 + e^{-x}}` | match (fixed) |
| normal_pdf | `let z = (x - mu) / sigma; let num = (-z.powi(2) / 2.0).exp(); num / (sigma * TAU.sqrt())` | `z = \frac{x - \mu}{\sigma}`; `num = e^{\frac{-z^{2}}{2.0}}`; `\frac{num}{\sigma \cdot \sqrt{\tau}}` | match |
| lorentz_factor | `1.0 / (1.0 - (v / c).powi(2)).sqrt()` | `\frac{1.0}{\sqrt{1.0 - {\frac{v}{c}}^{2}}}` | match (fixed, findings 7 + 8) |
| shannon_bits | `-p * p.log2() - (1.0 - p) * (1.0 - p).log2()` | `-p \cdot \log_{2}\left(p\right) - (1.0 - p) \cdot \log_{2}\left(1.0 - p\right)` | match (fixed, finding 7) |
| compound_growth | `let growth = (1.0 + rate).powf(periods as f64); principal * growth` | `growth = (1.0 + rate)^{periods}`; `principal \cdot growth` | match |
| root_mean_square | `((a.powi(2) + b.powi(2)) / 2.0).sqrt()` | `\sqrt{\frac{a^{2} + b^{2}}{2.0}}` | match |
| escape_velocity | `(2.0 * 6.674e-11 * mass / radius).sqrt()` | `\sqrt{\frac{2.0 \cdot 6.674 \times 10^{-11} \cdot mass}{radius}}` | match (fixed, finding 2) |

## skips

| function | input | verdict |
|---|---|---|
| skip_strings | `format!` + `len()` | skip (correct) |
| skip_control_flow | `if a > b { a } else { b }` | skip (correct) |
| skip_bool_logic | `a > 0.0 && b > 0.0` | skip (correct) |
| skip_assignment | `acc = acc * 2.0` statement | skip (correct) |
| skip_match | `match a { v => v * 1.0 }` | skip (correct) |
| skip_empty | empty body | skip (correct, no KaTeX loader added) |

## weird

| function | input | generated TeX | verdict |
|---|---|---|---|
| qualified_calls | `f64::powi(a, 2) + f64::hypot(a, b)` | `a^{2} + \sqrt{a^{2} + b^{2}}` | match (UFCS handled) |
| deeply_nested | `((a + b) * (c - d)).sqrt().abs().powi(2)` | `(\left\|\sqrt{(a + b) \cdot (c - d)}\right\|)^{2}` | match (parens reduced, finding 7) |
| rem_in_hypot | `a.hypot(x % y)` | `\sqrt{a^{2} + (x \bmod y)^{2}}` | match |
| mega_chain | `let q = a * b + c / d; let r = q.hypot(f) - g.sqrt(); r.powf(h + a) * PI` | `q = a \cdot b + \frac{c}{d}`; `r = \sqrt{q^{2} + f^{2}} - \sqrt{g}`; `r^{h + a} \cdot \pi` | match |
| log_base | `x.log(2.0)` | `\log_{2.0}\left(x\right)` | match (fixed, finding 1) |
| nonmath_method_arithmetic | `s.len() + s.chars().count()` | not extracted | **not extracted** (pinned, finding 6) |
| turbofish_binding | `n + s.len() as f64` | not extracted | **not extracted** (pinned, finding 6) |

## Overall

Every rendered formula is a faithful, unambiguous rendering of the Rust
arithmetic: precedence round-trips, composite bases and exponents are
grouped correctly, greek/subscript prettification composes, and literals
read as numbers. The remaining accepted imperfections are stylistic
(`a - -b` terseness, bracket indexing, integer division shown as a
fraction) and three pinned non-extractions for constructs that cannot be
drawn — none of which the docs ever advertise.
