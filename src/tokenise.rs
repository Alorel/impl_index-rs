use proc_macro2::{Ident, Punct, Spacing, Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::Token;

use crate::{IndexMacro, IndexedPairing, MaybeIdent, Pairing, Pattern};

impl IndexMacro {
    pub fn into_token_stream(self) -> TokenStream {
        let Self {
            impl_mut,
            generics,
            idx_by,
            impl_for,
            output,
            pairings,
        } = self;

        let (g1, g2, g3) = if let Some(ref generics) = generics {
            let (g1, g2, g3) = generics.split_for_impl();
            (Some(g1), Some(g2), Some(g3))
        } else {
            (None, None, None)
        };

        let inline = if pairings.len() < 3 {
            Some(quote!(#[inline]))
        } else {
            None
        };

        let impl_mut = if impl_mut {
            let mut_pairings = pairings.iter().map(|pairing| IndexedPairing {
                pairing,
                idx_by: &idx_by,
                mut_ref: Some(<Token![mut]>::default()),
            });

            Some(quote! {
                #[automatically_derived]
                impl #g1 ::core::ops::IndexMut<#idx_by> for #impl_for #g2 #g3 {
                    #inline
                    fn index_mut(&mut self, index_macro_derived_index_input: #idx_by) -> &mut Self::Output {
                        match index_macro_derived_index_input {
                            #(#mut_pairings),*
                        }
                    }
                }
            })
        } else {
            None
        };

        let immut_pairings = pairings.into_iter().map(|pairing| IndexedPairing {
            pairing,
            idx_by: &idx_by,
            mut_ref: None,
        });

        quote! {
            #[automatically_derived]
            impl #g1 ::core::ops::Index<#idx_by> for #impl_for #g2 #g3 {
                type Output = #output;

                #inline
                fn index(&self, index_macro_derived_index_input: #idx_by) -> &Self::Output {
                    match index_macro_derived_index_input {
                        #(#immut_pairings),*
                    }
                }
            }

            #impl_mut
        }
    }
}

impl<P: AsRef<Pairing>, T: ToTokens> ToTokens for IndexedPairing<P, T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Pairing {
            enum_variant,
            struct_field,
        } = self.pairing.as_ref();
        let idx_by = &self.idx_by;
        let mut_ref = self.mut_ref;

        enum_variant.to_tokens(tokens, |variant, tokens| {
            idx_by.to_tokens(tokens);
            <Token![::]>::default().to_tokens(tokens);
            variant.to_tokens(tokens);
        });

        <Token![=>]>::default().to_tokens(tokens);

        tokens.append(Punct::new('&', Spacing::Joint));
        mut_ref.to_tokens(tokens);

        tokens.append(Ident::new("self", Span::call_site()));
        tokens.append(Punct::new('.', Spacing::Joint));
        struct_field.to_tokens(tokens, |ident, tokens| {
            ident.to_tokens(tokens);
        });
    }
}

impl<Other: ToTokens> MaybeIdent<Other> {
    pub fn to_tokens<F>(&self, tokens: &mut TokenStream, mut fmt_field: F)
    where
        F: FnMut(&Ident, &mut TokenStream),
    {
        match self {
            MaybeIdent::Ident(ident) => {
                fmt_field(ident, tokens);
            }
            MaybeIdent::Other(other) => {
                other.to_tokens(tokens);
            }
        }
    }
}

impl ToTokens for Pattern {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.pat.to_tokens(tokens);
        if let Some((ref if_token, ref ty)) = self.guard {
            if_token.to_tokens(tokens);
            ty.to_tokens(tokens);
        }
    }
}
