use crate::spec::syntax::{Spec, SpecField};

use std::str::FromStr;

use quote::{quote, ToTokens};

impl Spec {
    pub fn generate_dials_impl(self, output: &mut proc_macro2::TokenStream) -> syn::Result<()> {
        let Self { name, fields } = self;

        let (struct_total_bits, struct_container_ty) = {
            let total_bits = fields.iter().map(|f| f.size).sum::<usize>();
            let struct_total_bits = container_size_bits(name.span(), total_bits)?;

            (
                struct_total_bits,
                proc_macro2::Ident::new(&format!("u{}", struct_total_bits), name.span()),
            )
        };

        let all_ones_bitmask = proc_macro2::Literal::from_str(&format!(
            "0b{}",
            (0..struct_total_bits).map(|_| "1").collect::<String>()
        ))?;

        let consts: proc_macro2::TokenStream = fields
            .iter()
            .map(|f| f.const_bitmask_declaration(struct_total_bits, &struct_container_ty))
            .collect::<syn::Result<_>>()?;

        let impls: proc_macro2::TokenStream = fields
            .iter()
            .map(|f| f.getter_setter_declaration(struct_total_bits, &struct_container_ty))
            .collect::<syn::Result<_>>()?;

        quote! {
            #[repr(transparent)]
            pub struct #name (pub #struct_container_ty);
            impl #name {
                const ALL_ONES: #struct_container_ty = #all_ones_bitmask;
                #consts

                #impls
            }
        }
        .to_tokens(output);

        Ok(())
    }
}

impl SpecField {
    fn getter_setter_declaration(
        &self,
        struct_total_bits: usize,
        struct_container_ty: &proc_macro2::Ident,
    ) -> syn::Result<proc_macro2::TokenStream> {
        let mut output = proc_macro2::TokenStream::new();
        let fld = &self.name;
        let fld_const_id =
            proc_macro2::Ident::new(&self.name.to_string().to_uppercase(), self.name.span());
        let fld_const_id_st = proc_macro2::Ident::new(
            &format!("{}_START", self.name.to_string().to_uppercase()),
            self.name.span(),
        );
        let set_fld = proc_macro2::Ident::new(&format!("set_{}", self.name), self.name.span());

        if self.size == 1 {
            let unset_fld =
                proc_macro2::Ident::new(&format!("unset_{}", self.name), self.name.span());
            let toggle_fld =
                proc_macro2::Ident::new(&format!("toggle_{}", self.name), self.name.span());

            quote! {
                pub fn #fld(&self) -> bool {
                    (self.0 >> Self::#fld_const_id_st) & 1 == 1
                }
                pub fn #set_fld(&mut self) -> &mut Self {
                    self.0 |= Self::#fld_const_id;
                    self
                }
                pub fn #unset_fld(&mut self) -> &mut Self {
                    self.0 &= Self::#fld_const_id;
                    self
                }
                pub fn #toggle_fld(&mut self) -> &mut Self {
                    self.0 ^= Self::#fld_const_id;
                    self
                }
            }
            .to_tokens(&mut output)
        } else {
            let fld_const_id_sz = proc_macro2::Ident::new(
                &format!("{}_SIZE", self.name.to_string().to_uppercase()),
                self.name.span(),
            );
            let lit_struct_total_bits =
                proc_macro2::Literal::from_str(&struct_total_bits.to_string())?;
            let lit_2_as_struct_container_type =
                proc_macro2::Literal::from_str(&format!("2_{struct_container_ty}"))?;
            quote! {
                pub fn #fld(&self) -> #struct_container_ty {
                    (self.0 >> Self::#fld_const_id_st) & (Self::ALL_ONES >> (#lit_struct_total_bits - Self::#fld_const_id_sz))
                }
                pub fn #set_fld(&mut self, mut value: #struct_container_ty) -> &mut Self {
                    if value >= #lit_2_as_struct_container_type.pow(Self::#fld_const_id_sz as u32) {
                        value = value % #lit_2_as_struct_container_type.pow(Self::#fld_const_id_sz as u32);
                    }
                    let mask = Self::ALL_ONES ^ Self::#fld_const_id;

                    (*self).0 = (self.0 & mask) | (value << Self::#fld_const_id_st);
                    self
                }
            }
            .to_tokens(&mut output)
        };
        Ok(output)
    }

    fn const_bitmask_declaration(
        &self,
        struct_total_bits: usize,
        struct_container_ty: &proc_macro2::Ident,
    ) -> syn::Result<proc_macro2::TokenStream> {
        let mut output = proc_macro2::TokenStream::new();
        let lit_bitmask = proc_macro2::Literal::from_str(&format!(
            "0b{}",
            (0..struct_total_bits)
                .rev()
                .map(|idx| {
                    if idx >= self.start && idx < self.start + self.size {
                        "1"
                    } else {
                        "0"
                    }
                })
                .collect::<String>()
        ))?;
        let const_name =
            proc_macro2::Ident::new(&self.name.to_string().to_uppercase(), self.name.span());

        quote! {
            pub const #const_name: #struct_container_ty = #lit_bitmask;
        }
        .to_tokens(&mut output);

        self.lit_to_associated_const("start")?
            .to_tokens(&mut output);
        self.lit_to_associated_const("size")?.to_tokens(&mut output);

        Ok(output)
    }

    fn lit_to_associated_const(&self, modifier: &str) -> syn::Result<proc_macro2::TokenStream> {
        let const_mod = proc_macro2::Ident::new(
            &format!(
                "{}_{}",
                self.name.to_string().to_uppercase(),
                modifier.to_uppercase()
            ),
            self.name.span(),
        );
        let lit = proc_macro2::Literal::from_str(
            &{
                if modifier == "start" {
                    self.start
                } else if modifier == "size" {
                    self.size
                } else {
                    return Err(syn::Error::new(
                        self.name.span(),
                        &format!("not implemented for {}", modifier),
                    ));
                }
            }
            .to_string(),
        )?;
        Ok(quote! {
            pub const #const_mod: usize = #lit;
        })
    }
}

fn container_size_bits(span: proc_macro2::Span, total_bits: usize) -> syn::Result<usize> {
    Ok(match total_bits {
        s if s <= 8 => 8,
        s if s <= 16 => 16,
        s if s <= 32 => 32,
        s if s <= 64 => 64,
        s if s <= 128 => 128,
        _ => {
            return Err(syn::Error::new(
                span,
                "The resultant structure is larger than 128 bits, which is the largest primitive type",
            ));
        }
    })
}
