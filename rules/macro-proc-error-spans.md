---
id: macro-proc-error-spans
title: Report proc-macro errors as spanned compile errors, never by panicking
category: macros
severity: warn
tier: 1
enforceability: judge
tags: [proc-macro, diagnostics, panic, syn]
applies_to:
  - "panic!/unwrap/expect inside a #[proc_macro_derive] or #[proc_macro]"
  - "a proc-macro that validates its input"
rust_edition: "2024"
msrv: null
detect:
  clippy: []
  grep: []
  ast_query: null
autofix: false
sources:
  - leonardomso/rust-skills#macro-proc-error-spans
supersedes: []
status: stable
reviewed: "2026-07-12"
---

## Rule
Signal invalid input in a proc-macro by returning a `syn::Error` converted to a compile error, never by `panic!`, `.unwrap()`, or `.expect()`.

## Why
A panic inside a proc-macro produces the opaque message "proc macro panicked" with no pointer into the user's source, so they cannot tell which code triggered it. Returning `Error::new_spanned(tokens, "message").to_compile_error()` instead emits a diagnostic that points directly at the offending span, exactly like an ordinary compiler error, and lets you report several problems at once with `Error::combine`.

## Do
```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Error};

#[proc_macro_derive(MyTrait)]
pub fn derive_my_trait(input: TokenStream) -> TokenStream {
    derive_inner(input).unwrap_or_else(|e| e.to_compile_error().into())
}

fn derive_inner(input: TokenStream) -> Result<TokenStream, Error> {
    let input: DeriveInput = syn::parse(input)?;
    let fields = match &input.data {
        Data::Struct(s) => &s.fields,
        _ => {
            return Err(Error::new_spanned(
                &input.ident,
                "MyTrait can only be derived on structs",
            ));
        }
    };
    let _first = fields
        .iter()
        .next()
        .ok_or_else(|| Error::new_spanned(&input.ident, "MyTrait requires at least one field"))?;
    let name = &input.ident;
    Ok(quote! { impl MyTrait for #name {} }.into())
}
```

## Don't
```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(MyTrait)]
pub fn derive_my_trait(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let fields = match input.data {
        Data::Struct(ref s) => &s.fields,
        _ => panic!("MyTrait can only be derived on structs"),
    };
    let first = fields.iter().next().unwrap();
    let name = first.ident.as_ref().unwrap();
    quote! { impl MyTrait for #name {} }.into()
}
```

## Notes
Wrap the logic in a `fn(...) -> Result<TokenStream, syn::Error>` and convert once at the entry point. Use `Error::new(span, msg)` when you only hold a `Span`. Keep messages lowercase with no trailing punctuation, matching compiler style.
