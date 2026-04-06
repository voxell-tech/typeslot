#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::punctuated::Punctuated;
use syn::{DeriveInput, Path, Token};

#[proc_macro_derive(TypeSlot, attributes(slot))]
pub fn derive_has_slot(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let slot_attr = input
        .attrs
        .iter()
        .find(|a| a.path().is_ident("slot"))
        .expect(
            "#[derive(TypeSlot)] requires a #[slot(...)] attribute",
        );

    let groups: Vec<Path> = slot_attr
        .parse_args_with(
            Punctuated::<Path, Token![,]>::parse_terminated,
        )
        .expect("#[slot(...)] expects comma-separated types")
        .into_iter()
        .collect();

    let impls = groups.iter().map(|group| {
        quote! {
            const _: () = {
                static __SLOT: ::typeslot::AtomicSlot =
                    ::typeslot::AtomicSlot::new();

                impl ::typeslot::TypeSlot<#group> for #name {
                    fn slot() -> Option<usize> {
                        __SLOT.get()
                    }
                }

                ::typeslot::inventory::submit! {
                    ::typeslot::TypeSlotEntry {
                        type_id: ::core::any::TypeId::of::<#name>(),
                        group_id: ::core::any::TypeId::of::<#group>(),
                        slot: &__SLOT,
                    }
                }
            };
        }
    });

    quote! { #(#impls)* }.into()
}
