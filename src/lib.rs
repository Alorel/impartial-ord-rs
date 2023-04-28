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
//! impl PartialOrd for MyStruct where Self: Ord {
//!   #[inline]
//!   fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//!     Some(Ord::cmp(self, other))
//!   }
//! }
//! ```

use quote::quote;
use syn::{parse2, parse_macro_input, DeriveInput, WherePredicate};

/// A quicker [PartialOrd](core::cmp::PartialOrd) for types that already implement
/// [Ord](core::cmp::Ord).
#[proc_macro_derive(ImpartialOrd)]
pub fn derive_partial_ord(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        ident,
        mut generics,
        ..
    } = parse_macro_input!(input as DeriveInput);

    let (gen_impl, gen_type, gen_where) = {
        let self_predicate = match parse2::<WherePredicate>(quote! { Self: ::core::cmp::Ord }) {
            Ok(p) => p,
            Err(e) => return e.to_compile_error().into(),
        };
        generics.make_where_clause().predicates.push(self_predicate);
        generics.split_for_impl()
    };

    (quote! {
        #[automatically_derived]
        impl #gen_impl ::core::cmp::PartialOrd for #ident #gen_type #gen_where {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering> {
                Some(::core::cmp::Ord::cmp(self, other))
            }
        }
    })
    .into()
}
