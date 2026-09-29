//! KaTeX-rendered math formulas in your rustdoc, extracted from function
//! bodies and doc strings.
//!
//! Attach [`formula_doc`] to a function:
//!
//! - every formula-shaped expression in its body — bindings with arithmetic
//!   right-hand sides, trailing and `return` expressions — is lifted into a
//!   `Formulas` section in the doc comment, and
//! - `$$expr$$` markers and ` ```formula ` fences in the existing doc string
//!   are rewritten in place: the marker content is parsed as a Rust
//!   expression and emitted as typeset display math.
//!
//! ```
//! use document_formulas::formula_doc;
//!
//! /// Half the base times the height.
//! ///
//! /// $$ area = base * height / 2 $$
//! #[formula_doc]
//! fn triangle_area(base: f64, height: f64) -> f64 {
//!     let half = 0.5;
//!     base * height * half
//! }
//! ```
//!
//! The generated docs embed a KaTeX loader (CSS + auto-render from a CDN),
//! so `cargo doc --open` and docs.rs display the formulas without any
//! further setup. Content that does not parse as a Rust expression is left
//! untouched.

mod extract;
mod latex;

use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::{Attribute, Expr, ItemFn, Lit, Meta, parse_macro_input};

const KATEX_VERSION: &str = "0.16.22";

/// Documents the math formulas contained in a function body and doc string.
///
/// Body extraction and doc-string rewriting run together; the attribute
/// currently takes no arguments.
#[proc_macro_attribute]
pub fn formula_doc(attr: TokenStream, item: TokenStream) -> TokenStream {
    if !attr.is_empty() {
        return syn::Error::new(
            Span::call_site().into(),
            "#[formula_doc] does not accept any arguments",
        )
        .to_compile_error()
        .into();
    }
    let mut func = parse_macro_input!(item as ItemFn);

    let doc_formulas = rewrite_doc_formulas(&mut func.attrs);

    let rendered: Vec<String> = extract::extract_formulas(&func)
        .iter()
        .filter_map(latex::formula_tex)
        .collect();

    if doc_formulas > 0 || !rendered.is_empty() {
        let loader = katex_loader();
        func.attrs.push(syn::parse_quote!(#[doc = #loader]));
    }
    if !rendered.is_empty() {
        let doc = build_doc(&rendered);
        func.attrs.push(syn::parse_quote!(#[doc = #doc]));
    }
    quote!(#func).into()
}

/// Rewrites `$$expr$$` lines and `formula` fences inside `#[doc]` attributes
/// into verbatim HTML blocks carrying KaTeX display math. Returns the number
/// of formulas produced; unparseable content is left untouched.
fn rewrite_doc_formulas(attrs: &mut [Attribute]) -> usize {
    let mut count = 0;
    for attr in attrs.iter_mut() {
        let Some(text) = doc_text(attr) else {
            continue;
        };
        let lines: Vec<&str> = text.split('\n').collect();
        let Some(rewritten) = rewrite_doc_lines(&lines) else {
            continue;
        };
        count += rewritten.iter().filter(|l| l.starts_with("$$ ")).count();
        let new_text = rewritten.join("\n");
        *attr = syn::parse_quote!(#[doc = #new_text]);
    }
    count
}

fn doc_text(attr: &Attribute) -> Option<String> {
    let Meta::NameValue(nv) = &attr.meta else {
        return None;
    };
    if !nv.path.is_ident("doc") {
        return None;
    }
    let Expr::Lit(lit) = &nv.value else {
        return None;
    };
    let Lit::Str(s) = &lit.lit else {
        return None;
    };
    Some(s.value())
}

fn rewrite_doc_lines(lines: &[&str]) -> Option<Vec<String>> {
    let mut out: Vec<String> = Vec::with_capacity(lines.len() + 8);
    let mut changed = false;
    let mut i = 0;
    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.starts_with("```") && trimmed[3..].trim() == "formula" {
            match lines[i + 1..].iter().position(|l| l.trim() == "```") {
                Some(end) if end > 0 => {
                    let joined = lines[i + 1..i + 1 + end].join(" ");
                    if let Some(tex) = formula_tex_str(&joined) {
                        out.extend(html_block(&format!("$$ {tex} $$")));
                        changed = true;
                        i += end + 2;
                        continue;
                    }
                }
                _ => {}
            }
        } else if trimmed.starts_with("$$")
            && trimmed.ends_with("$$")
            && trimmed.len() > 4
            && let Some(tex) = formula_tex_str(&trimmed[2..trimmed.len() - 2])
        {
            out.extend(html_block(&format!("$$ {tex} $$")));
            changed = true;
            i += 1;
            continue;
        }
        out.push(lines[i].to_string());
        i += 1;
    }
    changed.then_some(out)
}

/// The emitted lines form a CommonMark HTML block (type 6), which rustdoc
/// copies through verbatim — protecting TeX backslashes from markdown escape
/// processing. Surrounding blank lines terminate the block so following
/// prose stays normal markdown.
fn html_block(display_line: &str) -> impl Iterator<Item = String> {
    [
        "",
        "<div class=\"document-formulas\">",
        display_line,
        "</div>",
        "",
    ]
    .into_iter()
    .map(String::from)
}

fn formula_tex_str(code: &str) -> Option<String> {
    latex::to_tex(&syn::parse_str::<Expr>(code.trim()).ok()?)
}

fn build_doc(formulas: &[String]) -> String {
    let mut doc = String::new();
    doc.push('\n');
    doc.push_str(&katex_loader());
    doc.push_str("\n\n# Formulas\n\n");
    // The formulas are wrapped in an HTML block so rustdoc passes the TeX
    // through verbatim; markdown escape processing would otherwise eat the
    // backslashes in sequences like `\_`.
    doc.push_str("<div class=\"document-formulas\">\n");
    for tex in formulas {
        doc.push_str("$$ ");
        doc.push_str(tex);
        doc.push_str(" $$\n");
    }
    doc.push_str("</div>\n");
    doc
}

fn katex_loader() -> String {
    format!(
        concat!(
            "<link rel=\"stylesheet\" href=\"{cdn}@{v}/dist/katex.min.css\" />\n",
            "<script defer src=\"{cdn}@{v}/dist/katex.min.js\"></script>\n",
            "<script defer src=\"{cdn}@{v}/dist/contrib/auto-render.min.js\" ",
            "onload=\"if (!window.documentFormulasKatexLoaded) {{ window.documentFormulasKatexLoaded = true; ",
            "renderMathInElement(document.body, {{ delimiters: [",
            "{{ left: '$$', right: '$$', display: true }}, ",
            "{{ left: '\\\\(', right: '\\\\)', display: false }}] }}); }}\"></script>\n",
        ),
        cdn = "https://cdn.jsdelivr.net/npm/katex",
        v = KATEX_VERSION,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doc_wraps_formulas_in_html_block() {
        let doc = build_doc(&["flight\\_time = v_{0}".to_string()]);
        assert!(doc.contains("<div class=\"document-formulas\">\n"));
        assert!(doc.contains("$$ flight\\_time = v_{0} $$\n"));
        assert!(doc.trim_end().ends_with("</div>"));
        assert!(doc.contains("katex.min.css"));
    }

    #[test]
    fn rewrites_display_marker_in_place() {
        let lines = [
            "Some prose.",
            "",
            "$$ area = base * height / 2 $$",
            "More prose.",
        ];
        let out = rewrite_doc_lines(&lines).unwrap();
        let joined = out.join("\n");
        assert!(joined.contains("$$ area = \\frac{base \\cdot height}{2} $$"));
        assert!(joined.contains("<div class=\"document-formulas\">"));
        assert!(joined.contains("Some prose."));
        assert!(joined.contains("More prose."));
        // HTML block must be terminated by a blank line so following prose
        // is not swallowed into it.
        let div_end = out.iter().position(|l| l == "</div>").unwrap();
        assert_eq!(out[div_end + 1], "");
    }

    #[test]
    fn rewrites_formula_fence() {
        let lines = [
            "Leading.",
            "",
            "```formula",
            "a.powi(2) + b",
            "```",
            "",
            "Trailing.",
        ];
        let out = rewrite_doc_lines(&lines).unwrap();
        let joined = out.join("\n");
        assert!(!joined.contains("```"));
        assert!(joined.contains("$$ a^{2} + b $$"));
        assert!(joined.contains("Trailing."));
    }

    #[test]
    fn leaves_unparseable_content_untouched() {
        // Nothing parseable -> no rewrite happens at all; mixed cases are
        // covered by `counts_only_emitted_formulas`, which asserts the
        // unparseable marker passes through verbatim next to a rewritten one.
        let lines = ["$$foo bar baz$$", "```formula", "??? ###", "```"];
        assert!(rewrite_doc_lines(&lines).is_none());
    }

    #[test]
    fn unchanged_doc_yields_none() {
        let lines = ["Plain prose.", "* markdown *", "- list"];
        assert!(rewrite_doc_lines(&lines).is_none());
    }

    #[test]
    fn unterminated_fence_is_left_alone() {
        let lines = ["```formula", "a + b"];
        assert!(rewrite_doc_lines(&lines).is_none());
    }

    #[test]
    fn counts_only_emitted_formulas() {
        let mut attrs: Vec<Attribute> = syn::parse_quote! {
            #[doc = "Text.\n\n$$ a / b $$\n\n$$not parseable$$"]
        };
        assert_eq!(rewrite_doc_formulas(&mut attrs), 1);
        let Attribute { meta, .. } = &attrs[0];
        let Meta::NameValue(nv) = meta else {
            panic!("not a name-value attr");
        };
        let Expr::Lit(lit) = &nv.value else {
            panic!("not a literal");
        };
        let Lit::Str(s) = &lit.lit else {
            panic!("not a string");
        };
        assert!(s.value().contains("\\frac{a}{b}"));
        assert!(s.value().contains("$$not parseable$$"));
    }
}
