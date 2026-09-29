//! KaTeX-rendered math formulas in your rustdoc, extracted from function
//! bodies.
//!
//! Attach [`formula_doc`] to a function: every formula-shaped expression in
//! its body — bindings with arithmetic right-hand sides, trailing and
//! `return` expressions — is lifted into a `Formulas` section in the doc
//! comment.
//!
//! ```
//! use document_formulas::formula_doc;
//!
//! /// Half the base times the height.
//! #[formula_doc]
//! fn triangle_area(base: f64, height: f64) -> f64 {
//!     let half = 0.5;
//!     base * height * half
//! }
//! ```
//!
//! The generated docs embed a KaTeX loader (CSS + auto-render from a CDN),
//! so `cargo doc --open` and docs.rs display the formulas without any
//! further setup.

mod extract;
mod latex;

use proc_macro::{Span, TokenStream};
use quote::quote;
use syn::{ItemFn, parse_macro_input};

const KATEX_VERSION: &str = "0.16.22";

/// Documents the math formulas contained in a function body.
///
/// The attribute currently takes no arguments.
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

    let rendered: Vec<String> = extract::extract_formulas(&func)
        .iter()
        .filter_map(latex::formula_tex)
        .collect();

    if !rendered.is_empty() {
        let loader = katex_loader();
        func.attrs.push(syn::parse_quote!(#[doc = #loader]));
        let doc = build_doc(&rendered);
        func.attrs.push(syn::parse_quote!(#[doc = #doc]));
    }
    quote!(#func).into()
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
}
