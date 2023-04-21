//! ```
//! # use core::cmp::{PartialOrd, Ord, Ordering};
//! # type Bar = u8;
//! # type Baz = u8;
//! #
//! // Input
//! #[derive(PartialEq, Eq, Ord, impartial_ord::ImpartialOrd)]
//! # struct Impostor;
//! # #[derive(PartialEq, Eq, Ord)]
//! struct MyStruct { foo: Bar, qux: Baz, }
//!
//! // Output
//! impl PartialOrd for MyStruct {
//!   #[inline]
//!   fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//!     Some(Ord::cmp(self, other))
//!   }
//! }
//! ```

use proc_macro2::{Punct, Spacing, TokenStream};
use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, DeriveInput, GenericParam, Generics, Token};

/// A quicker [PartialOrd](core::cmp::PartialOrd) for types that already implement
/// [Ord](core::cmp::Ord).
#[proc_macro_derive(ImpartialOrd)]
pub fn derive_partial_ord(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let gen_suffix = {
        let (_, gen_type, gen_where) = input.generics.split_for_impl();
        quote! { #gen_type #gen_where }
    };
    let struct_name = &input.ident;

    let gen_impl = render_generics(input.generics);

    (quote! {
        #[automatically_derived]
        impl #gen_impl ::core::cmp::PartialOrd for #struct_name #gen_suffix {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering> {
                Some(::core::cmp::Ord::cmp(self, other))
            }
        }
    })
    .into()
}

#[inline(always)]
fn render_generics(generics: Generics) -> TokenStream {
    if generics.params.is_empty() {
        return TokenStream::new();
    }

    let lt = &generics.lt_token;
    let rt = &generics.gt_token;

    let body = generics
        .params
        .into_iter()
        .map(move |f| match f {
            GenericParam::Type(mut t) => {
                t.default = None;
                let p = Punct::new(
                    if t.colon_token.is_some() { '+' } else { ':' },
                    Spacing::Alone,
                );

                quote! { #t #p ::core::cmp::Ord }
            }
            GenericParam::Const(mut t) => {
                t.default = None;

                t.into_token_stream()
            }
            other => other.into_token_stream(),
        })
        .collect::<Punctuated<TokenStream, Token![,]>>();

    quote! { #lt #body #rt }
}
