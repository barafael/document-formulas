//! Extraction of formula-shaped expressions from function bodies.
//!
//! A candidate lifts only when it is formula-shaped (arithmetic or known
//! math calls) *and* renders, so the docs can never advertise a formula
//! that silently vanishes.

use syn::{Expr, ItemFn, Pat, Stmt};

use crate::latex;

pub struct Formula {
    pub name: Option<String>,
    pub tex: String,
}

pub fn extract_formulas(func: &ItemFn) -> Vec<Formula> {
    let mut formulas = Vec::new();
    for stmt in &func.block.stmts {
        match stmt {
            Stmt::Local(local) => {
                let Some(init) = &local.init else {
                    continue;
                };
                if init.diverge.is_some() {
                    continue;
                }
                let Pat::Ident(binding) = &local.pat else {
                    continue;
                };
                if latex::is_math_like(&init.expr)
                    && let Some(tex) = latex::to_tex(&init.expr)
                {
                    formulas.push(Formula {
                        name: Some(binding.ident.to_string()),
                        tex,
                    });
                }
            }
            Stmt::Expr(Expr::Return(ret), _) => {
                if let Some(expr) = &ret.expr
                    && latex::is_math_like(expr)
                    && let Some(tex) = latex::to_tex(expr)
                {
                    formulas.push(Formula { name: None, tex });
                }
            }
            Stmt::Expr(expr, _) => {
                if latex::is_math_like(expr)
                    && let Some(tex) = latex::to_tex(expr)
                {
                    formulas.push(Formula { name: None, tex });
                }
            }
            _ => {}
        }
    }
    formulas
}

#[cfg(test)]
mod tests {
    use super::*;

    fn formulas_of(src: &str) -> Vec<Formula> {
        extract_formulas(&syn::parse_str::<ItemFn>(src).unwrap())
    }

    #[test]
    fn extracts_bindings_and_trailing_expr() {
        let fs = formulas_of("fn f(a: f64, b: f64) -> f64 { let c = a + b; c * 2.0 }");
        assert_eq!(fs.len(), 2);
        assert_eq!(fs[0].name.as_deref(), Some("c"));
        assert_eq!(fs[0].tex, "a + b");
        assert_eq!(fs[1].name, None);
        assert_eq!(fs[1].tex, "c \\cdot 2.0");
    }

    #[test]
    fn skips_plain_code() {
        let fs = formulas_of(
            r#"fn f(s: &str) -> usize {
                let n = s.len();
                let msg = format!("{n}");
                println!("{msg}");
                n
            }"#,
        );
        assert!(fs.is_empty());
    }

    #[test]
    fn skips_unrenderable_arithmetic() {
        let fs = formulas_of("fn f(s: &str) -> usize { s.len() + 1 }");
        assert!(fs.is_empty());
    }

    #[test]
    fn extracts_tuple_fields() {
        let fs = formulas_of("fn f(t: (f64, f64)) -> f64 { t.0 + t.1 }");
        assert_eq!(fs.len(), 1);
        assert_eq!(fs[0].tex, "t_{0} + t_{1}");
    }

    #[test]
    fn extracts_return_expr() {
        let fs = formulas_of("fn f(a: f64) -> f64 { return a * 2.0; }");
        assert_eq!(fs.len(), 1);
        assert_eq!(fs[0].name, None);
    }
}
