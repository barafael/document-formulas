//! Extraction of formula-shaped expressions from function bodies.

use syn::{Expr, ItemFn, Pat, Stmt};

use crate::latex::is_math_like;

pub struct Formula {
    pub name: Option<String>,
    pub expr: Expr,
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
                if is_math_like(&init.expr) {
                    formulas.push(Formula {
                        name: Some(binding.ident.to_string()),
                        expr: (*init.expr).clone(),
                    });
                }
            }
            Stmt::Expr(Expr::Return(ret), _) => {
                if let Some(expr) = &ret.expr
                    && is_math_like(expr)
                {
                    formulas.push(Formula {
                        name: None,
                        expr: (**expr).clone(),
                    });
                }
            }
            Stmt::Expr(expr, _) if is_math_like(expr) => {
                formulas.push(Formula {
                    name: None,
                    expr: expr.clone(),
                });
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
        assert_eq!(fs[1].name, None);
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
    fn extracts_return_expr() {
        let fs = formulas_of("fn f(a: f64) -> f64 { return a * 2.0; }");
        assert_eq!(fs.len(), 1);
        assert_eq!(fs[0].name, None);
    }
}
