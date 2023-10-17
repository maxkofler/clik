use std::collections::HashMap;

use syn::{punctuated::Punctuated, spanned::Spanned, Attribute, Expr, Lit, Token};

use crate::arg::Arg;

/// Parses an attribute into the `args` Vec
pub fn parse_attr<'a>(
    attr: &Attribute,
    args: &mut HashMap<String, Arg<'a>>,
) -> Result<(), proc_macro2::TokenStream> {
    let mut nested = attr
        .parse_args_with(Punctuated::<Expr, Token![,]>::parse_terminated)
        .unwrap();
    if nested.len() != 2 {
        return Err(syn::Error::new(
            attr.span(),
            "Expected exactly 2 arguments: (<argument>, <description>)",
        )
        .to_compile_error());
    }

    let second = nested.pop().expect("Expected second arg to exist");
    let first = nested.pop().expect("Expected first arg to exist");

    let second = match second.value() {
        Expr::Lit(l) => match &l.lit {
            Lit::Str(l) => l.value(),
            _ => {
                return Err(
                    syn::Error::new(second.span(), "Expected literal string").to_compile_error()
                );
            }
        },
        _ => {
            return Err(
                syn::Error::new(second.span(), "Expected literal string").to_compile_error()
            );
        }
    };

    match first.value() {
        Expr::Path(p) => {
            let ident = p.path.require_ident().unwrap();
            match args.get_mut(&ident.to_string()) {
                Some(i) => {
                    i.help = Some(second);
                }
                None => {
                    return Err(
                        syn::Error::new(first.span(), "Describing non-existing argument")
                            .to_compile_error(),
                    );
                }
            }
        }
        _ => {
            return Err(syn::Error::new(first.span(), "Expected ident").to_compile_error());
        }
    }

    Ok(())
}
