---
id: macro-proc-syn-quote
title: Build procedural macros with syn, quote, and proc-macro2
category: macros
severity: warn
tier: 1
enforceability: judge
tags: [proc-macro, syn, quote, tooling]
applies_to:
  - "a #[proc_macro_derive] or #[proc_macro] implementation"
  - "hand-iterating proc_macro::TokenStream to parse input"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#macro-proc-syn-quote
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Parse proc-macro input with `syn`, generate output with `quote`, and use `proc-macro2` token types, rather than hand-iterating `proc_macro::TokenStream`.

## Why
Hand-parsing a raw `TokenStream` is fragile and verbose and breaks on generics, where-clauses, and attributes. The `syn`/`quote`/`proc-macro2` trio gives you a typed AST, readable quasi-quoting, and span-aware tokens usable outside the compiler, so you can unit-test generation logic in ordinary `#[test]` functions. Enable only the `syn` features you use — `derive` suffices for most derives; `full` parses all syntax at a compile-time cost.

## Do
```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Hello)]
pub fn derive_hello(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    quote! {
        impl #impl_generics Hello for #name #ty_generics #where_clause {
            fn hello(&self) {
                println!("hello from {}", stringify!(#name));
            }
        }
    }
    .into()
}
```

## Don't
```rust
use proc_macro::TokenStream;

#[proc_macro_derive(Hello)]
pub fn derive_hello(input: TokenStream) -> TokenStream {
    let mut iter = input.into_iter();
    iter.next();
    let name = iter.next().unwrap().to_string();
    format!("impl Hello for {name} {{ fn hello(&self) {{}} }}")
        .parse()
        .unwrap()
}
```

## Notes
Attach generated code to meaningful spans with `quote_spanned! { field.span() => ... }` so downstream errors point at the right place. Because `proc-macro2::TokenStream` works outside the compiler, factor generation into a plain function and test it with `syn::parse_quote!`.
