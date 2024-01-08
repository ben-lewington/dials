extern crate proc_macro;

mod spec;

#[proc_macro]
pub fn spec(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    spec::generate_dials(input.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
