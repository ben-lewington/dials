use crate::spec::syntax::{Spec, SpecField};

use proc_macro2::{Span, TokenTree};

pub struct SpecParser(pub proc_macro2::token_stream::IntoIter);

impl SpecParser {
    pub fn new(input: proc_macro2::TokenStream) -> Self {
        Self(input.into_iter())
    }

    pub fn parse(&mut self) -> syn::Result<Spec> {
        let name = match self.peek2() {
            (Some(TokenTree::Ident(s)), Some(TokenTree::Ident(name))) if s == "struct" => {
                self.advance2();
                name
            }
            t => {
                return Err(syn::Error::new(
                    t.0.map(|t| t.span()).unwrap_or(Span::mixed_site()),
                    "expected struct declaration",
                ));
            }
        };

        let mut g = SpecParser(match self.advance1() {
            Some(TokenTree::Group(g)) => g.stream().into_iter(),
            _ => {
                return Err(syn::Error::new(
                    name.span().into(),
                    "expected braced declaration of struct fields",
                ))
            }
        });
        let mut fields = vec![];
        let mut start: usize = 0;
        loop {
            match g.peek2() {
                (Some(TokenTree::Ident(field)), Some(TokenTree::Punct(p)))
                    if p.as_char() == ':' =>
                {
                    g.advance2();
                    match g.peek1() {
                        Some(TokenTree::Ident(ty)) if ty == "bool" || ty.to_string().starts_with('u') => {
                            let size = if ty == "bool" {
                                1
                            } else {
                                let Ok(s) = ty.to_string()[1..].parse::<usize>() else {
                                    return Err(syn::Error::new(
                                        ty.span().into(),
                                        "expected a type declaration of the form `bool` or `u{N}`, where N is a usize",
                                    ));
                                };
                                s
                            };
                            fields.push(SpecField { name: field, start, size });

                            start += size;

                            g.advance1();
                            match self.peek1() {
                                Some(TokenTree::Punct(p)) if p.as_char() == ',' => {}
                                None => {}
                                _ => {
                                    return Err(syn::Error::new(
                                        ty.span().into(),
                                        "Expected ',' or end of field declarations",
                                    ));
                                }
                            }
                            g.advance1();
                        }
                        Some(TokenTree::Ident(ty)) => {
                            return Err(syn::Error::new(
                                ty.span().into(),
                                "expected a type declaration of the form `bool` or `u{N}`, where N is a usize",
                            ))
                        }
                        t => {
                            todo!("{t:?}");
                        }
                    }
                }
                (None, None) => break Ok(Spec { name, fields }),
                _ => todo!("a {:?}", g.peek3()),
            }
        }
    }

    fn peek1(&self) -> Option<TokenTree> {
        self.0.clone().next()
    }

    fn peek2(&self) -> (Option<TokenTree>, Option<TokenTree>) {
        let mut it = self.0.clone();
        let x0 = it.next();
        (x0, it.next())
    }

    fn peek3(&self) -> (Option<TokenTree>, Option<TokenTree>, Option<TokenTree>) {
        let mut it = self.0.clone();
        let x0 = it.next();
        let x1 = it.next();
        (x0, x1, it.next())
    }

    fn advance1(&mut self) -> Option<TokenTree> {
        self.0.next()
    }

    fn advance2(&mut self) -> (Option<TokenTree>, Option<TokenTree>) {
        let x0 = self.0.next();
        (x0, self.0.next())
    }
}
