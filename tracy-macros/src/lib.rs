extern crate proc_macro;

use quote::quote;
use syn::{parse_macro_input, ItemFn, Stmt};

use crate::proc_macro::{TokenStream, TokenTree};

fn zone_scoped_common(item: TokenStream, prefix: Option<String>) -> TokenStream {
    let mut ast = parse_macro_input!(item as ItemFn);
    let ident = &ast.sig.ident;
    let prefix = prefix.unwrap_or("".to_string());

    let prologue = quote! {
        let _zone_scoped = {
            const name: &'static str = concat!(#prefix, stringify!(#ident), "\0");
            const file: &'static str = concat!(file!(), "\0");
            const srcloc: rustracy::SourceLocation = rustracy::SourceLocation {
                name: name.as_ptr() as *const _,
                function: name.as_ptr() as *const _,
                file: file.as_ptr() as *const _,
                line: line!(),
                color: 0,
            };
            rustracy::ZoneScoped::new(&srcloc)
        };
    };
    let prologue: TokenStream = prologue.into();
    let prologue = parse_macro_input!(prologue as Stmt);

    ast.block.stmts.insert(0, prologue);

    let gen = quote! {
        #ast
    };
    gen.into()
}

/// Logs a `zone`.
///
/// ```ignore
/// #[zone_scoped]
//  fn render_frame() { /* ... */ }
/// ```
#[proc_macro_attribute]
pub fn zone_scoped(_attr: TokenStream, item: TokenStream) -> TokenStream {
    zone_scoped_common(item, None)
}

/// Logs a `zone` with a prefix.
///
/// ```ignore
/// struct Engine { /* ...*/ };
/// impl Engine {
///     #[zone_scoped_prefix(Engine)]
///     fn render_frame() { /* ... */ }
/// }
/// ```
#[proc_macro_attribute]
pub fn zone_scoped_prefix(attr: TokenStream, item: TokenStream) -> TokenStream {
    let prefix = attr.into_iter().find_map(|token| match token {
        TokenTree::Ident(ident) => Some(format!("{}::", ident.to_string())),
        _ => None,
    });
    zone_scoped_common(item, prefix)
}
