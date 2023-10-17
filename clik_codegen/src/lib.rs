#![doc = include_str!("../README.md")]
extern crate proc_macro;
extern crate proc_macro2;

mod arg;
mod attr;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    token::Comma,
    LitStr,
};

struct ClikCommandArgs {
    name: Ident,
    help: LitStr,
}

impl Parse for ClikCommandArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Comma>()?;
        let help: LitStr = input.parse()?;
        Ok(ClikCommandArgs { name, help })
    }
}

#[proc_macro_attribute]
#[doc = include_str!("../docs/clik_command.md")]
pub fn clik_command(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_args = syn::parse_macro_input!(attr as ClikCommandArgs);
    let command_name = input_args.name;
    let command_help = input_args.help;

    let input = syn::parse_macro_input!(input as syn::ItemFn);

    let args = match arg::parse_args(&input.sig.inputs, &input.attrs) {
        Ok(a) => a,
        Err(e) => return e.into(),
    };

    let state_var = match arg::get_state_var(&input.sig.inputs) {
        Ok(var) => var,
        Err(err) => return err.into(),
    };

    for attr in &input.attrs {
        if attr.path().is_ident("clik_arg") {}
    }

    let fn_name = &input.sig.ident;
    let new_fn_name = proc_macro2::Ident::new(&format!("function_{}", fn_name), Span::call_site());
    let state_type = &state_var;
    let arg_blocks = arg::create_parse_blocks(args);
    let body = &input.block;

    match input.sig.asyncness {
        // Synchronous function
        None => TokenStream::from(quote! {
            /// Construct a command struct containing the defined command
            fn #fn_name<'a>() -> clik::Command<'a, #state_type> {
                Command::new(stringify!(#command_name), stringify!(#command_help), #new_fn_name)
            }

            fn #new_fn_name(state: &mut #state_type, args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {

                // Parse all the command arguments
                #(#arg_blocks)*

                // The body of the function
                #body
            }
        }),
        // Async function
        Some(_) => TokenStream::from(quote! {
            /// Construct a command struct containing the defined command
            fn #fn_name<'a>() -> clik::Command<'a, #state_type> {
                Command::new_async(stringify!(#command_name), stringify!(#command_help), clik::async_fn!(#state_type, #new_fn_name))
            }

            async fn #new_fn_name(state: &mut #state_type, args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {

                // Parse all the command arguments
                #(#arg_blocks)*

                // The body of the function
                #body
            }
        }),
    }
}
