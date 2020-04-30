extern crate proc_macro;

use quote::quote;
use syn::{parse_macro_input, ItemFn, Stmt};

use crate::proc_macro::{TokenStream, TokenTree};

fn zone_scoped_common(item: TokenStream, prefix: Option<String>) -> TokenStream {
    let mut ast = parse_macro_input!(item as ItemFn);
    let ident = &ast.sig.ident;
    let prefix = prefix.unwrap_or("".to_string());

    let prologue = quote! {
        let _ctx = {
            const name: &'static str = concat!(#prefix, stringify!(#ident), "\0");
            const file: &'static str = concat!(file!(), "\0");
            const srcloc: rustracy::SourceLocation = rustracy::SourceLocation {
                name: name.as_ptr() as *const _,
                function: name.as_ptr() as *const _,
                file: file.as_ptr() as *const _,
                line: line!(),
                color: 0,
            };
            rustracy::emit_zone_begin(&srcloc)
        };
    };
    let prologue: TokenStream = prologue.into();
    let prologue = parse_macro_input!(prologue as Stmt);

    let mut body = quote! {};
    for s in &ast.block.stmts {
        body = quote! {
            #body
            #s
        };
    }
    let body = quote! {
        let mut body = || { #body };
    };
    let body: TokenStream = body.into();
    let body = parse_macro_input!(body as Stmt);

    let epilogue = quote! {
        {
            let result = body();
            rustracy::emit_zone_end(_ctx);
            result
        }
    };
    let epilogue: TokenStream = epilogue.into();
    let epilogue = parse_macro_input!(epilogue as Stmt);

    ast.block.stmts.clear();
    ast.block.stmts.push(prologue);
    ast.block.stmts.push(body);
    ast.block.stmts.push(epilogue);

    let gen = quote! {
        #ast
    };
    gen.into()
}

#[proc_macro_attribute]
pub fn zone_scoped(_attr: TokenStream, item: TokenStream) -> TokenStream {
    zone_scoped_common(item, None)
}

#[proc_macro_attribute]
pub fn zone_scoped_prefix(attr: TokenStream, item: TokenStream) -> TokenStream {
    let prefix = attr.into_iter().find_map(|token| match token {
        TokenTree::Ident(ident) => Some(format!("{}::", ident.to_string())),
        _ => None,
    });
    zone_scoped_common(item, prefix)
}
