use crate::quote;
use std::collections::HashMap;

use syn::{
    punctuated::Punctuated, spanned::Spanned, token::Comma, Attribute, FnArg, Pat, PatType, Type,
    TypePath,
};

use crate::attr;

#[derive(Debug)]
pub struct Arg<'a> {
    pub pos: usize,
    pub arg: &'a PatType,
    pub help: Option<String>,
}

/// Parses out the arguments to the command from the function signature
/// # Arguments
/// * `args` - The arguments supplied to the function
/// * `attr` - The attributes for the function to parse `clik_arg`, etc...
pub fn parse_args<'a>(
    args: &'a Punctuated<FnArg, Comma>,
    attr: &Vec<Attribute>,
) -> Result<Vec<Arg<'a>>, proc_macro2::TokenStream> {
    let mut res: HashMap<String, Arg<'a>> = HashMap::new();

    // Parse args out
    for (i, arg) in args.iter().enumerate() {
        if i == 0 {
            continue;
        }
        if let syn::FnArg::Typed(arg) = arg {
            let name = match arg.pat.as_ref() {
                Pat::Ident(i) => i.ident.to_string(),
                _ => {
                    return Err(
                        syn::Error::new(arg.pat.span(), "Expected ident").into_compile_error()
                    )
                }
            };
            res.insert(
                name,
                Arg {
                    pos: res.len(),
                    arg: arg,
                    help: None,
                },
            );
        }
    }

    // Now parse attrs for descriptions
    for attr in attr {
        if attr.path().is_ident("clik_arg") {
            attr::parse_attr(&attr, &mut res)?
        }
    }

    // Convert the hashmap to a vector and sort it
    let mut vec: Vec<Arg<'a>> = res.into_iter().map(|f| f.1).collect();
    vec.sort_by(|a, b| a.pos.cmp(&b.pos));
    Ok(vec)
}

/// Retrieves the state variable type from the arguments
///
/// This ensures that there is at least 1 argument (the state variable)
/// and that it is a `&mut` reference.
///
/// If there are any errors, they get output in the form of a compile error packed into a `TokenStream`
pub fn get_state_var<'a>(
    args: &'a Punctuated<FnArg, Comma>,
) -> Result<TypePath, proc_macro2::TokenStream> {
    Err(match args.first() {
        Some(first) => match first {
            FnArg::Typed(typed) => match typed.ty.as_ref() {
                Type::Reference(reference) => match reference.elem.as_ref() {
                    Type::Path(p) => return Ok(p.clone()),
                    _ => syn::Error::new(typed.span(), "State variable must be a type"),
                },
                _ => syn::Error::new(typed.span(), "State variable must be a reference"),
            },
            FnArg::Receiver(r) => syn::Error::new(r.span(), "State variable cannot be 'self'"),
        },
        None => syn::Error::new(
            args.span(),
            "Expected at least one argument: state variable",
        ),
    }
    .into_compile_error())
}

/// Takes in an argument vector and creates a vector of TokenStreams
/// from them. These TokenStreams contain the parsing blocks for each argument
/// # Arguments
/// * `args` - The vector of arguments to transform
pub fn create_parse_blocks<'a>(args: Vec<Arg<'a>>) -> Vec<proc_macro2::TokenStream> {
    let mut res: Vec<proc_macro2::TokenStream> = Vec::new();

    for arg in args {
        let ty = &arg.arg.ty;
        let ident = &arg.arg.pat;
        let i = res.len();

        let block = quote! {
            let #ident: #ty = {
                match match args.get(#i) {
                    None => return Err(
                        clik::error::MissingArgumentError {
                            name: stringify!(#ident).to_string(),
                            position: #i,
                            ty: stringify!(#ty).to_string()
                        }.into()),
                    Some(v) => v
                }.parse::<#ty>() {
                    Ok(v) => v,
                    Err(e) => return Err(
                        clik::error::WrongArgumentError {
                            name: stringify!(#ident).to_string(),
                            position: #i,
                            ty: stringify!(#ty).to_string(),
                            inner: e.into()
                        }.into())
                }
            };
        };

        res.push(block);
    }

    res
}
