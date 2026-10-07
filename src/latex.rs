//! Translation of Rust arithmetic expressions into KaTeX-flavoured TeX.

use syn::{
    BinOp, Expr, ExprBinary, ExprCall, ExprMethodCall, Lit, Member, Path, PathArguments, UnOp,
};

use crate::extract::Formula;

const PREC_CMP: u8 = 0;
const PREC_ADD: u8 = 1;
const PREC_MUL: u8 = 2;
const PREC_UNARY: u8 = 3;
const PREC_POSTFIX: u8 = 4;
const PREC_ATOM: u8 = 5;

const CONSTANTS: &[(&str, &str)] = &[("PI", "\\pi"), ("TAU", "\\tau"), ("E", "e")];

const GREEK: &[(&str, &str)] = &[
    ("alpha", "\\alpha"),
    ("beta", "\\beta"),
    ("gamma", "\\gamma"),
    ("delta", "\\delta"),
    ("epsilon", "\\epsilon"),
    ("zeta", "\\zeta"),
    ("eta", "\\eta"),
    ("theta", "\\theta"),
    ("iota", "\\iota"),
    ("kappa", "\\kappa"),
    ("lambda", "\\lambda"),
    ("mu", "\\mu"),
    ("nu", "\\nu"),
    ("xi", "\\xi"),
    ("pi", "\\pi"),
    ("rho", "\\rho"),
    ("sigma", "\\sigma"),
    ("tau", "\\tau"),
    ("upsilon", "\\upsilon"),
    ("phi", "\\phi"),
    ("chi", "\\chi"),
    ("psi", "\\psi"),
    ("omega", "\\omega"),
];

const UNARY_MATH_METHODS: &[&str] = &[
    "sqrt", "cbrt", "abs", "recip", "exp", "ln", "log", "log2", "log10", "sin", "cos", "tan",
    "asin", "acos", "atan", "sinh", "cosh", "tanh", "asinh", "acosh", "atanh", "floor", "ceil",
    "round",
];

const BINARY_MATH_METHODS: &[&str] = &["powi", "powf", "pow", "atan2", "hypot", "min", "max"];

const FREE_MATH_FNS: &[&str] = &[
    "sqrt", "cbrt", "abs", "recip", "exp", "ln", "log", "log2", "log10", "sin", "cos", "tan",
    "asin", "acos", "atan", "sinh", "cosh", "tanh", "floor", "ceil", "round", "powi", "powf",
    "pow", "atan2", "hypot", "min", "max",
];

/// Formats a full `name = expr` formula line.
pub fn formula_tex(formula: &Formula) -> String {
    match &formula.name {
        Some(name) => format!("{} = {}", ident_tex(name), formula.tex),
        None => formula.tex.clone(),
    }
}

/// Converts an expression to TeX, or `None` if it contains unrenderable parts.
pub fn to_tex(expr: &Expr) -> Option<String> {
    emit(expr, PREC_CMP)
}

/// True if the expression contains arithmetic operators or known math calls;
/// the shape gate deciding whether an expression is formula material at all.
pub fn is_math_like(expr: &Expr) -> bool {
    match expr {
        Expr::Binary(b) => {
            matches!(
                b.op,
                BinOp::Add(_) | BinOp::Sub(_) | BinOp::Mul(_) | BinOp::Div(_) | BinOp::Rem(_)
            ) || is_math_like(&b.left)
                || is_math_like(&b.right)
        }
        Expr::MethodCall(m) => {
            is_math_method(&m.method.to_string())
                || is_math_like(&m.receiver)
                || m.args.iter().any(is_math_like)
        }
        Expr::Call(c) => path_is_math_free(&c.func) || c.args.iter().any(is_math_like),
        Expr::Paren(p) => is_math_like(&p.expr),
        Expr::Group(g) => is_math_like(&g.expr),
        Expr::Unary(u) => is_math_like(&u.expr),
        Expr::Cast(c) => is_math_like(&c.expr),
        Expr::Tuple(t) => t.elems.iter().any(is_math_like),
        _ => false,
    }
}

pub fn is_math_method(name: &str) -> bool {
    UNARY_MATH_METHODS.contains(&name) || BINARY_MATH_METHODS.contains(&name)
}

fn path_is_math_free(expr: &Expr) -> bool {
    let Expr::Path(p) = expr else {
        return false;
    };
    let Some(seg) = p.path.segments.last() else {
        return false;
    };
    FREE_MATH_FNS.contains(&seg.ident.to_string().as_str())
}

fn emit(expr: &Expr, min_prec: u8) -> Option<String> {
    let (tex, prec) = emit_prec(expr)?;
    if prec < min_prec {
        Some(format!("({tex})"))
    } else {
        Some(tex)
    }
}

fn emit_all<'a>(exprs: impl IntoIterator<Item = &'a Expr>) -> Option<Vec<String>> {
    exprs.into_iter().map(|e| emit(e, 0)).collect()
}

fn emit_prec(expr: &Expr) -> Option<(String, u8)> {
    match expr {
        Expr::Lit(l) => lit_tex(&l.lit),
        Expr::Path(p) => path_tex(&p.path),
        Expr::Binary(b) => bin_tex(b),
        Expr::Unary(u) => match u.op {
            UnOp::Neg(_) => {
                let inner = emit(&u.expr, PREC_UNARY)?;
                Some((format!("-{inner}"), PREC_UNARY))
            }
            _ => None,
        },
        Expr::Paren(p) => emit_prec(&p.expr),
        Expr::Group(g) => emit_prec(&g.expr),
        Expr::Cast(c) => emit_prec(&c.expr),
        Expr::MethodCall(m) => method_tex(m),
        Expr::Call(c) => call_tex(c),
        Expr::Field(f) => {
            let base = emit(&f.base, PREC_POSTFIX)?;
            match &f.member {
                Member::Named(name) => Some((
                    format!("{base}.{}", ident_tex(&name.to_string())),
                    PREC_POSTFIX,
                )),
                Member::Unnamed(index) => {
                    Some((format!("{base}_{{{}}}", index.index), PREC_POSTFIX))
                }
            }
        }
        Expr::Index(i) => {
            let base = emit(&i.expr, PREC_POSTFIX)?;
            let idx = emit(&i.index, 0)?;
            Some((format!("{base}[{idx}]"), PREC_POSTFIX))
        }
        Expr::Tuple(t) => {
            let items = emit_all(&t.elems)?;
            Some((format!("\\left({}\\right)", items.join(", ")), PREC_ATOM))
        }
        _ => None,
    }
}

fn bin_tex(b: &ExprBinary) -> Option<(String, u8)> {
    let (op, prec, left_req, right_req) = match b.op {
        BinOp::Add(_) => (" + ", PREC_ADD, PREC_ADD, PREC_ADD),
        BinOp::Sub(_) => (" - ", PREC_ADD, PREC_ADD, PREC_MUL),
        BinOp::Mul(_) => (" \\cdot ", PREC_MUL, PREC_MUL, PREC_UNARY),
        BinOp::Div(_) => {
            let num = strip_redundant_parens(&emit(&b.left, 0)?);
            let den = strip_redundant_parens(&emit(&b.right, 0)?);
            return Some((format!("\\frac{{{num}}}{{{den}}}"), PREC_POSTFIX));
        }
        BinOp::Rem(_) => (" \\bmod ", PREC_MUL, PREC_UNARY, PREC_UNARY),
        BinOp::Eq(_) => (" = ", PREC_CMP, PREC_ADD, PREC_ADD),
        BinOp::Ne(_) => (" \\neq ", PREC_CMP, PREC_ADD, PREC_ADD),
        BinOp::Lt(_) => (" < ", PREC_CMP, PREC_ADD, PREC_ADD),
        BinOp::Gt(_) => (" > ", PREC_CMP, PREC_ADD, PREC_ADD),
        BinOp::Le(_) => (" \\le ", PREC_CMP, PREC_ADD, PREC_ADD),
        BinOp::Ge(_) => (" \\ge ", PREC_CMP, PREC_ADD, PREC_ADD),
        _ => return None,
    };
    let left = emit(&b.left, left_req)?;
    let right = emit(&b.right, right_req)?;
    Some((format!("{left}{op}{right}"), prec))
}

fn method_tex(m: &ExprMethodCall) -> Option<(String, u8)> {
    let name = m.method.to_string();
    if !is_math_method(&name) || m.turbofish.is_some() {
        return None;
    }
    let mut ops = vec![strip_redundant_parens(&emit(&m.receiver, PREC_POSTFIX)?)];
    for arg in &m.args {
        ops.push(emit(arg, 0)?);
    }
    math_tex(&name, &ops)
}

fn call_tex(c: &ExprCall) -> Option<(String, u8)> {
    let Expr::Path(p) = &*c.func else {
        return None;
    };
    let seg = p.path.segments.last()?;
    if !matches!(seg.arguments, PathArguments::None) {
        return None;
    }
    let name = seg.ident.to_string();
    if !FREE_MATH_FNS.contains(&name.as_str()) {
        return None;
    }
    math_tex(&name, &emit_all(&c.args)?)
}

fn math_tex(name: &str, ops: &[String]) -> Option<(String, u8)> {
    if let Some(t) = trig_name(name) {
        let [x] = ops else {
            return None;
        };
        return Some((
            format!("{t}\\left({}\\right)", strip_redundant_parens(x)),
            PREC_POSTFIX,
        ));
    }
    match name {
        "powi" | "powf" | "pow" => {
            let [x, y] = ops else {
                return None;
            };
            Some((
                format!("{}^{{{}}}", scripted_operand(x), strip_redundant_parens(y)),
                PREC_POSTFIX,
            ))
        }
        "sqrt" => {
            let [x] = ops else {
                return None;
            };
            Some((
                format!("\\sqrt{{{}}}", strip_redundant_parens(x)),
                PREC_POSTFIX,
            ))
        }
        "cbrt" => {
            let [x] = ops else {
                return None;
            };
            Some((
                format!("\\sqrt[3]{{{}}}", strip_redundant_parens(x)),
                PREC_POSTFIX,
            ))
        }
        "abs" => {
            let [x] = ops else {
                return None;
            };
            Some((
                format!("\\left|{}\\right|", strip_redundant_parens(x)),
                PREC_POSTFIX,
            ))
        }
        "recip" => {
            let [x] = ops else {
                return None;
            };
            Some((
                format!("\\frac{{1}}{{{}}}", strip_redundant_parens(x)),
                PREC_POSTFIX,
            ))
        }
        "exp" => {
            let [x] = ops else {
                return None;
            };
            Some((format!("e^{{{}}}", strip_redundant_parens(x)), PREC_POSTFIX))
        }
        "ln" | "log2" | "log10" => {
            let [x] = ops else {
                return None;
            };
            let head = match name {
                "ln" => "\\ln",
                "log2" => "\\log_{2}",
                _ => "\\log_{10}",
            };
            Some((
                format!("{head}\\left({}\\right)", strip_redundant_parens(x)),
                PREC_POSTFIX,
            ))
        }
        "log" => match ops {
            [x] => Some((
                format!("\\log\\left({}\\right)", strip_redundant_parens(x)),
                PREC_POSTFIX,
            )),
            [x, base] => Some((
                format!(
                    "\\log_{{{base}}}\\left({}\\right)",
                    strip_redundant_parens(x)
                ),
                PREC_POSTFIX,
            )),
            _ => None,
        },
        "floor" => {
            let [x] = ops else {
                return None;
            };
            Some((
                format!(
                    "\\left\\lfloor {}\\right\\rfloor",
                    strip_redundant_parens(x)
                ),
                PREC_POSTFIX,
            ))
        }
        "ceil" => {
            let [x] = ops else {
                return None;
            };
            Some((
                format!("\\left\\lceil {}\\right\\rceil", strip_redundant_parens(x)),
                PREC_POSTFIX,
            ))
        }
        "round" => {
            let [x] = ops else {
                return None;
            };
            Some((
                format!(
                    "\\operatorname{{round}}\\left({}\\right)",
                    strip_redundant_parens(x)
                ),
                PREC_POSTFIX,
            ))
        }
        "atan2" => {
            let [x, y] = ops else {
                return None;
            };
            Some((
                format!(
                    "\\operatorname{{atan2}}\\left({}, {}\\right)",
                    strip_redundant_parens(x),
                    strip_redundant_parens(y)
                ),
                PREC_POSTFIX,
            ))
        }
        "hypot" => {
            let [x, y] = ops else {
                return None;
            };
            Some((
                format!(
                    "\\sqrt{{{}^{{2}} + {}^{{2}}}}",
                    scripted_operand(x),
                    scripted_operand(y)
                ),
                PREC_POSTFIX,
            ))
        }
        "min" | "max" => {
            let [x, y] = ops else {
                return None;
            };
            Some((
                format!(
                    "\\{name}\\left({}, {}\\right)",
                    strip_redundant_parens(x),
                    strip_redundant_parens(y)
                ),
                PREC_POSTFIX,
            ))
        }
        _ => None,
    }
}

fn trig_name(name: &str) -> Option<&'static str> {
    Some(match name {
        "sin" => "\\sin",
        "cos" => "\\cos",
        "tan" => "\\tan",
        "asin" => "\\arcsin",
        "acos" => "\\arccos",
        "atan" => "\\arctan",
        "sinh" => "\\sinh",
        "cosh" => "\\cosh",
        "tanh" => "\\tanh",
        "asinh" => "\\operatorname{asinh}",
        "acosh" => "\\operatorname{acosh}",
        "atanh" => "\\operatorname{atanh}",
        _ => return None,
    })
}

/// Strips a redundant outer parenthesis group; slots that already delimit
/// their contents (`\sqrt{}`, `\frac{}{}`, `^{}`, `\left(\right)`) do not
/// need another layer.
fn strip_redundant_parens(s: &str) -> String {
    if is_parenthesized(s) {
        s[1..s.len() - 1].to_string()
    } else {
        s.to_string()
    }
}

/// True if the string is one balanced `( ... )` group.
fn is_parenthesized(s: &str) -> bool {
    let Some(inner) = s.strip_prefix('(').and_then(|r| r.strip_suffix(')')) else {
        return false;
    };
    let mut depth = 1usize;
    for c in inner.chars() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return false;
                }
            }
            _ => {}
        }
    }
    depth == 1
}

/// Adapts an operand for a trailing superscript: parenthesized groups and
/// plain atoms pass through, `\frac`/`\sqrt` atoms get braced, and operands
/// that already carry a superscript or consist of several parts get wrapped
/// in an explicit group.
fn scripted_operand(s: &str) -> String {
    if is_parenthesized(s) {
        s.to_string()
    } else if s.contains('^') {
        format!("\\left({s}\\right)")
    } else if s.starts_with("\\frac") || s.starts_with("\\sqrt") {
        format!("{{{s}}}")
    } else if s.contains(' ') {
        format!("({s})")
    } else {
        s.to_string()
    }
}

fn path_tex(path: &Path) -> Option<(String, u8)> {
    let last = path.segments.last()?;
    if !matches!(last.arguments, PathArguments::None) {
        return None;
    }
    let name = last.ident.to_string();
    let is_const = CONSTANTS.iter().any(|(n, _)| *n == name);
    if path.segments.len() > 1 && !is_const {
        return None;
    }
    Some((ident_tex(&name), PREC_ATOM))
}

pub fn ident_tex(name: &str) -> String {
    if let Some((_, tex)) = CONSTANTS.iter().find(|(n, _)| *n == name) {
        return (*tex).to_string();
    }
    if let Some((_, tex)) = GREEK.iter().find(|(n, _)| *n == name) {
        return (*tex).to_string();
    }
    if name.contains('_') {
        return name.replace('_', "\\_");
    }
    let base = name.trim_end_matches(|c: char| c.is_ascii_digit());
    if !base.is_empty() && base.len() != name.len() {
        let digits = &name[base.len()..];
        if let Some((_, tex)) = GREEK.iter().find(|(n, _)| *n == base) {
            return format!("{tex}_{{{digits}}}");
        }
        return format!("{base}_{{{digits}}}");
    }
    name.to_string()
}

fn lit_tex(lit: &Lit) -> Option<(String, u8)> {
    match lit {
        Lit::Int(i) => Some((strip_num(i.to_string()), PREC_ATOM)),
        Lit::Float(f) => Some((strip_num(f.to_string()), PREC_ATOM)),
        _ => None,
    }
}

fn strip_num(literal: String) -> String {
    let mut s = literal.replace('_', "");
    for suffix in [
        "f64", "f32", "u128", "u64", "u32", "u16", "u8", "i128", "i64", "i32", "i16", "i8",
        "usize", "isize",
    ] {
        if let Some(stripped) = s.strip_suffix(suffix) {
            if stripped
                .chars()
                .last()
                .is_some_and(|c| c.is_ascii_digit() || c == '.')
            {
                s = stripped.to_string();
            }
            break;
        }
    }
    if s.starts_with("0x") || s.starts_with("0o") || s.starts_with("0b") {
        return format!("\\mathtt{{{s}}}");
    }
    if let Some((mantissa, exponent)) = split_exponent(&s) {
        return format!("{mantissa} \\times 10^{{{exponent}}}");
    }
    s
}

/// Splits a decimal literal into mantissa and exponent if it uses scientific
/// notation (`1.5e-8`, `3E+4`).
fn split_exponent(s: &str) -> Option<(&str, &str)> {
    let pos = s.find(['e', 'E'])?;
    let (mantissa, rest) = (&s[..pos], &s[pos + 1..]);
    let exponent = rest.strip_prefix('+').unwrap_or(rest);
    let digits = exponent.strip_prefix('-').unwrap_or(exponent);
    if !mantissa.is_empty() && !digits.is_empty() && digits.chars().all(|c| c.is_ascii_digit()) {
        Some((mantissa, exponent))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tex(src: &str) -> Option<String> {
        to_tex(&syn::parse_str(src).unwrap())
    }

    #[test]
    fn methods() {
        assert_eq!(tex("a.powi(2)").as_deref(), Some(r"a^{2}"));
        assert_eq!(tex("sum.sqrt()").as_deref(), Some(r"\sqrt{sum}"));
        assert_eq!(tex("total.abs()").as_deref(), Some(r"\left|total\right|"));
        assert_eq!(tex("x.exp()").as_deref(), Some("e^{x}"));
        assert_eq!(
            tex("x.log10()").as_deref(),
            Some(r"\log_{10}\left(x\right)")
        );
        assert_eq!(
            tex("x.floor()").as_deref(),
            Some(r"\left\lfloor x\right\rfloor")
        );
        assert_eq!(
            tex("y.atan2(x)").as_deref(),
            Some(r"\operatorname{atan2}\left(y, x\right)")
        );
        assert_eq!(tex("a.hypot(b)").as_deref(), Some(r"\sqrt{a^{2} + b^{2}}"));
    }

    #[test]
    fn operators() {
        assert_eq!(tex("a / b").as_deref(), Some(r"\frac{a}{b}"));
        assert_eq!(tex("(a + b) / 2.0").as_deref(), Some(r"\frac{a + b}{2.0}"));
        assert_eq!(tex("PI * r * r").as_deref(), Some(r"\pi \cdot r \cdot r"));
        assert_eq!(tex("x % 2").as_deref(), Some(r"x \bmod 2"));
        assert_eq!(tex("a2 + b2").as_deref(), Some("a_{2} + b_{2}"));
        assert_eq!(tex("x == y").as_deref(), Some("x = y"));
        assert_eq!(tex("-x").as_deref(), Some("-x"));
        assert_eq!(
            tex("1_000_000u64 / 2").as_deref(),
            Some(r"\frac{1000000}{2}")
        );
    }

    #[test]
    fn identifiers_and_constants() {
        assert_eq!(tex("two_a + 1").as_deref(), Some(r"two\_a + 1"));
        assert_eq!(tex("std::f64::consts::PI").as_deref(), Some(r"\pi"));
        assert_eq!(tex("theta / 2.0").as_deref(), Some(r"\frac{\theta}{2.0}"));
        assert_eq!(tex("self.width").as_deref(), Some("self.width"));
        assert_eq!(
            tex("self.process_noise").as_deref(),
            Some(r"self.process\_noise")
        );
    }

    #[test]
    fn grouping_and_script_chains() {
        assert_eq!(tex("(a + b).powi(2)").as_deref(), Some("(a + b)^{2}"));
        assert_eq!(
            tex("a.hypot(b + c)").as_deref(),
            Some(r"\sqrt{a^{2} + (b + c)^{2}}")
        );
        assert_eq!(tex("pow(a + b, 2)").as_deref(), Some("(a + b)^{2}"));
        assert_eq!(
            tex("a.powi(2).powi(3)").as_deref(),
            Some(r"\left(a^{2}\right)^{3}")
        );
        assert_eq!(
            tex("(-b + root) / two_a").as_deref(),
            Some(r"\frac{-b + root}{two\_a}")
        );
    }

    #[test]
    fn logarithm_with_base() {
        assert_eq!(
            tex("x.log(2.0)").as_deref(),
            Some(r"\log_{2.0}\left(x\right)")
        );
        assert_eq!(
            tex("x.log(b + 1.0)").as_deref(),
            Some(r"\log_{b + 1.0}\left(x\right)")
        );
        assert_eq!(tex("log(x)").as_deref(), Some(r"\log\left(x\right)"));
        assert_eq!(
            tex("log(x, 2.0)").as_deref(),
            Some(r"\log_{2.0}\left(x\right)")
        );
    }

    #[test]
    fn tuple_fields_render_as_subscripts() {
        assert_eq!(tex("t.0 + t.1").as_deref(), Some("t_{0} + t_{1}"));
        assert_eq!(tex("m.12 * scale").as_deref(), Some(r"m_{12} \cdot scale"));
    }

    #[test]
    fn literal_forms() {
        assert_eq!(
            tex("1.5e-8 * a").as_deref(),
            Some(r"1.5 \times 10^{-8} \cdot a")
        );
        assert_eq!(tex("1e3 * a").as_deref(), Some(r"1 \times 10^{3} \cdot a"));
        assert_eq!(tex("3E+4 + a").as_deref(), Some(r"3 \times 10^{4} + a"));
        assert_eq!(
            tex("0x10 as f64 * a").as_deref(),
            Some(r"\mathtt{0x10} \cdot a")
        );
        assert_eq!(
            tex("0b1010 as f64 * a").as_deref(),
            Some(r"\mathtt{0b1010} \cdot a")
        );
        assert_eq!(
            tex("0o17 as f64 * a").as_deref(),
            Some(r"\mathtt{0o17} \cdot a")
        );
    }

    #[test]
    fn greek_composes_with_subscripts() {
        assert_eq!(tex("theta2 + 1.0").as_deref(), Some(r"\theta_{2} + 1.0"));
        assert_eq!(
            tex("lambda1 * lambda2").as_deref(),
            Some(r"\lambda_{1} \cdot \lambda_{2}")
        );
        assert_eq!(tex("pi * 2.0").as_deref(), Some(r"\pi \cdot 2.0"));
        assert_eq!(tex("tau * 2.0").as_deref(), Some(r"\tau \cdot 2.0"));
        assert_eq!(
            tex("two_a + 1").as_deref(),
            Some(r"two\_a + 1"),
            "underscored names keep escaping and gain no subscript"
        );
    }

    #[test]
    fn paren_hygiene() {
        assert_eq!(tex("(a + b).sqrt()").as_deref(), Some(r"\sqrt{a + b}"));
        assert_eq!(
            tex("(a + b).ln()").as_deref(),
            Some(r"\ln\left(a + b\right)")
        );
        assert_eq!(tex("(-a).abs()").as_deref(), Some(r"\left|-a\right|"));
        assert_eq!(tex("(-b).exp()").as_deref(), Some("e^{-b}"));
        assert_eq!(tex("(a + b).exp()").as_deref(), Some("e^{a + b}"));
        assert_eq!(tex("(a + b).recip()").as_deref(), Some(r"\frac{1}{a + b}"));
        assert_eq!(
            tex("(2.0 * theta).sin()").as_deref(),
            Some(r"\sin\left(2.0 \cdot \theta\right)")
        );
        assert_eq!(
            tex("(1.0 - p).log2()").as_deref(),
            Some(r"\log_{2}\left(1.0 - p\right)")
        );
    }

    #[test]
    fn exponents_brace_frac_and_sqrt_bases() {
        assert_eq!(
            tex("(a / b).powi(2)").as_deref(),
            Some(r"{\frac{a}{b}}^{2}")
        );
        assert_eq!(tex("sqrt(a).powi(2)").as_deref(), Some(r"{\sqrt{a}}^{2}"));
        assert_eq!(
            tex("a.hypot(b.hypot(c))").as_deref(),
            Some(r"\sqrt{a^{2} + \left(\sqrt{b^{2} + c^{2}}\right)^{2}}")
        );
    }

    #[test]
    fn non_math_bails() {
        assert_eq!(tex("s.len()"), None);
        assert_eq!(tex("foo(a) + 1.0"), None);
        assert_eq!(tex("\"a string\""), None);
        assert_eq!(tex("s.len() + 1"), None);
    }
}
