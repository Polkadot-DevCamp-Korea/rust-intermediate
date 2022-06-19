
mod construct_runtime;
use proc_macro::TokenStream;

#[proc_macro]
pub fn construct_runtime(input: TokenStream) -> TokenStream {
    construct_runtime::construct_runtime(input)
}