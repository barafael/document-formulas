# document_formulas

[![crates.io](https://img.shields.io/crates/v/document_formulas.svg)](https://crates.io/crates/document_formulas)
[![docs.rs](https://img.shields.io/docsrs/document_formulas)](https://docs.rs/document_formulas)
[![docs (workspace)](https://img.shields.io/badge/docs-rendered-8da0cb)](https://barafael.github.io/document-formulas/)

KaTeX-rendered math formulas in your rustdoc, extracted from function bodies.

Attach `#[formula_doc]` to a function and every formula-shaped expression in
its body — bindings with arithmetic right-hand sides, trailing and `return`
expressions — is lifted into the doc comment as a TeX formula. The attribute
embeds a KaTeX loader (CSS + auto-render from a CDN), so the formulas render
in `cargo doc --open` and on [docs.rs](https://docs.rs/document_formulas) without
any further setup.

## Usage

```toml
[dependencies]
document_formulas = "0.2"
```

```rust
use document_formulas::formula_doc;

/// Computes the hypotenuse of a right-angled triangle.
#[formula_doc]
pub fn hypotenuse(a: f64, b: f64) -> f64 {
    let a2 = a.powi(2);
    let b2 = b.powi(2);
    let sum = a2 + b2;
    sum.sqrt()
}
```

The generated documentation renders as:

- $$\mathrm{a_{2}} = \mathrm{a^{2}}$$
- $$\mathrm{sum} = \mathrm{a_{2}} + \mathrm{b_{2}}$$
- $$\sqrt{\mathrm{sum}}$$

## Supported constructs

- arithmetic: `+`, `-`, `*` (`\cdot`), `/` (`\frac`), `%` (`\bmod`), unary `-`, comparisons
- common methods: `powi`/`powf` (exponent), `sqrt`, `cbrt`, `abs`, `recip`, `exp`, `ln`, `log`,
  `log2`, `log10`, trigonometry (`sin`, `atan2`, ...), `hypot`, `min`/`max`, `floor`, `ceil`, `round`
- constants: `PI` (`\pi`), `TAU` (`\tau`), `E` (`e`); greek-letter identifiers (`theta` → `\theta`)
- identifier prettification: trailing digits become subscripts (`a2` → `a₂`)

Anything that cannot be rendered is skipped, so the macro never breaks your build.

## Formulas in doc strings

The attribute also rewrites markers in the doc string itself, in place, using
the same expression language:

```rust
/// Years for money to double at a given annual rate (rule of 72).
///
/// $$ years = 72.0 / annual_rate $$
///
/// ```formula
/// effective = annual_rate - inflation
/// ```
#[formula_doc]
pub fn doubling_time(annual_rate: f64) -> f64 { /* ... */ }
```

Each marker becomes a typeset equation exactly where you wrote it. Content
that does not parse as a Rust expression is left untouched.

## Examples

Example crates in this repository, with their formulas rendered live:

- [`kalman`](examples/kalman) — scalar 1D Kalman filter —
  [rendered](https://barafael.github.io/document-formulas/kalman/struct.Kalman1D.html)
- [`projectile`](examples/projectile) — ballistics closed forms —
  [rendered](https://barafael.github.io/document-formulas/projectile/fn.range.html)
- [`finance`](examples/finance) — compound interest and amortization —
  [rendered](https://barafael.github.io/document-formulas/finance/fn.monthly_payment.html)

The full workspace documentation is deployed on every push:
https://barafael.github.io/document-formulas/

## Notes

- Requires Rust 1.85+ (edition 2024).
- Viewing the docs requires network access to the KaTeX CDN
  (`cdn.jsdelivr.net`); the docs themselves build offline.

## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at
your option.
