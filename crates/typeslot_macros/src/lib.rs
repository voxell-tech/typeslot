#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::DeriveInput;
use syn::Path;
use syn::Token;
use syn::parse_macro_input;
use syn::punctuated::Punctuated;

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
        let group_ident = &group.segments.last().unwrap().ident;
        let slot_ident = format_ident!(
            "__TYPESLOT_{}_{}",
            name.to_string().to_uppercase(),
            group_ident.to_string().to_uppercase(),
        );

        quote! {
            static #slot_ident: ::typeslot::AtomicSlot =
                ::typeslot::AtomicSlot::new();

            impl ::typeslot::TypeSlot<#group> for #name {
                fn slot() -> Option<usize> {
                    #slot_ident.get()
                }
            }

            ::typeslot::inventory::submit! {
                ::typeslot::TypeSlotEntry {
                    type_id: ::core::any::TypeId::of::<#name>(),
                    group_id: ::core::any::TypeId::of::<#group>(),
                    slot: &#slot_ident,
                }
            }
        }
    });

    quote! { #(#impls)* }.into()
}
