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
    ("rho", "\\rho"),
    ("sigma", "\\sigma"),
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

/// Renders a full `name = expr` formula line, or `None` if unrenderable.
pub fn formula_tex(formula: &Formula) -> Option<String> {
    let tex = to_tex(&formula.expr)?;
    Some(match &formula.name {
        Some(name) => format!("{} = {}", ident_tex(name), tex),
        None => tex,
    })
}

/// Converts an expression to TeX, or `None` if it contains unrenderable parts.
pub fn to_tex(expr: &Expr) -> Option<String> {
    emit(expr, PREC_CMP)
}

/// True if the expression contains arithmetic operators or known math calls.
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
            let Member::Named(name) = &f.member else {
                return None;
            };
            let base = emit(&f.base, PREC_POSTFIX)?;
            Some((
                format!("{base}.{}", ident_tex(&name.to_string())),
                PREC_POSTFIX,
            ))
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
            let num = emit(&b.left, 0)?;
            let den = emit(&b.right, 0)?;
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
    let mut ops = vec![emit(&m.receiver, PREC_POSTFIX)?];
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
        return Some((format!("{t}\\left({x}\\right)"), PREC_POSTFIX));
    }
    match name {
        "powi" | "powf" | "pow" => {
            let [x, y] = ops else {
                return None;
            };
            let base = wrap_scripted(&wrap_composite(x));
            Some((format!("{base}^{{{y}}}"), PREC_POSTFIX))
        }
        "sqrt" => {
            let [x] = ops else {
                return None;
            };
            Some((format!("\\sqrt{{{x}}}"), PREC_POSTFIX))
        }
        "cbrt" => {
            let [x] = ops else {
                return None;
            };
            Some((format!("\\sqrt[3]{{{x}}}"), PREC_POSTFIX))
        }
        "abs" => {
            let [x] = ops else {
                return None;
            };
            Some((format!("\\left|{x}\\right|"), PREC_POSTFIX))
        }
        "recip" => {
            let [x] = ops else {
                return None;
            };
            Some((format!("\\frac{{1}}{{{x}}}"), PREC_POSTFIX))
        }
        "exp" => {
            let [x] = ops else {
                return None;
            };
            Some((format!("e^{{{x}}}"), PREC_POSTFIX))
        }
        "ln" | "log" | "log2" | "log10" => {
            let [x] = ops else {
                return None;
            };
            let head = match name {
                "ln" => "\\ln",
                "log" => "\\log",
                "log2" => "\\log_{2}",
                _ => "\\log_{10}",
            };
            Some((format!("{head}\\left({x}\\right)"), PREC_POSTFIX))
        }
        "floor" => {
            let [x] = ops else {
                return None;
            };
            Some((format!("\\left\\lfloor {x}\\right\\rfloor"), PREC_POSTFIX))
        }
        "ceil" => {
            let [x] = ops else {
                return None;
            };
            Some((format!("\\left\\lceil {x}\\right\\rceil"), PREC_POSTFIX))
        }
        "round" => {
            let [x] = ops else {
                return None;
            };
            Some((
                format!("\\operatorname{{round}}\\left({x}\\right)"),
                PREC_POSTFIX,
            ))
        }
        "atan2" => {
            let [x, y] = ops else {
                return None;
            };
            Some((
                format!("\\operatorname{{atan2}}\\left({x}, {y}\\right)"),
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
                    wrap_scripted(&wrap_composite(x)),
                    wrap_scripted(&wrap_composite(y))
                ),
                PREC_POSTFIX,
            ))
        }
        "min" | "max" => {
            let [x, y] = ops else {
                return None;
            };
            Some((format!("\\{name}\\left({x}, {y}\\right)"), PREC_POSTFIX))
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

/// Composite operands (containing binary operators) need parens when
/// spliced under a superscript, unless already parenthesized.
fn wrap_composite(operand: &str) -> String {
    if operand.contains(' ') && !is_parenthesized(operand) {
        format!("({operand})")
    } else {
        operand.to_string()
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

/// Bases that already carry a superscript need braces before taking another.
fn wrap_scripted(base: &str) -> String {
    if base.contains('^') {
        format!("{{{base}}}")
    } else {
        base.to_string()
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

fn ident_tex(name: &str) -> String {
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
        return format!("{base}_{{{}}}", &name[base.len()..]);
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
    s
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
        assert_eq!(tex("a.powi(2).powi(3)").as_deref(), Some("{a^{2}}^{3}"));
        assert_eq!(
            tex("(-b + root) / two_a").as_deref(),
            Some(r"\frac{-b + root}{two\_a}")
        );
    }

    #[test]
    fn non_math_bails() {
        assert_eq!(tex("s.len()"), None);
        assert_eq!(tex("foo(a) + 1.0"), None);
        assert_eq!(tex("\"a string\""), None);
    }
}
