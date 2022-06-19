
use proc_macro::TokenStream;
mod parse;

use parse::{RuntimeDeclaration};

pub fn construct_runtime(input: TokenStream) -> TokenStream {
    let input_copy = input.clone();
    let definition = syn::parse_macro_input!(input as RuntimeDeclaration);

    let res = match definition {

        RuntimeDeclaration::Implicit(implicit_def) => {

        }

        RuntimeDeclaration::Explicit(explicit_def) => {

        }
    };
    
    "test".parse().unwrap()
}