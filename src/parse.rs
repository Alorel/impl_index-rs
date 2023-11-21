use syn::parse::{Parse, ParseStream};
use syn::{custom_keyword, Generics, Pat, Token};

use crate::{IndexMacro, MaybeIdent, Pairing, Pattern};

impl Parse for IndexMacro {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let (generics, impl_for) = {
            if input.peek(Token![<]) {
                if input.fork().parse::<Generics>().is_ok() {
                    (Some(input.parse()?), input.parse()?)
                } else {
                    (None, input.parse()?)
                }
            } else {
                (None, input.parse()?)
            }
        };

        let idx_by = {
            custom_keyword!(by);
            input.parse::<by>()?;
            input.parse()?
        };

        input.parse::<Token![=>]>()?;

        let impl_mut = if input.peek(Token![mut]) {
            input.parse::<Token![mut]>()?;
            true
        } else {
            false
        };

        Ok(Self {
            impl_mut,
            generics,
            impl_for,
            idx_by,
            output: input.parse()?,
            pairings: {
                input.parse::<Token![:]>()?;
                input.parse_terminated(Pairing::parse, Token![,])?
            },
        })
    }
}

impl Parse for Pattern {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            pat: Pat::parse_multi_with_leading_vert(input)?,
            guard: {
                if input.peek(Token![if]) {
                    Some((input.parse()?, input.parse()?))
                } else {
                    None
                }
            },
        })
    }
}

impl<Other: Parse> Parse for MaybeIdent<Other> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        custom_keyword!(pat);

        if input.peek(pat) {
            input.parse::<pat>()?;
            Ok(Self::Other(input.parse()?))
        } else {
            Ok(Self::Ident(input.parse()?))
        }
    }
}

impl Parse for Pairing {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            enum_variant: input.parse()?,
            struct_field: {
                input.parse::<Token![=>]>()?;
                input.parse()?
            },
        })
    }
}
