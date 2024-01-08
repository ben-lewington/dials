mod generate;
mod parse;
mod syntax;

use crate::spec::parse::SpecParser;

pub fn generate_dials(input: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    let mut output = proc_macro2::TokenStream::new();

    SpecParser::new(input)
        .parse()?
        .generate_dials_impl(&mut output)?;

    Ok(output)
}
