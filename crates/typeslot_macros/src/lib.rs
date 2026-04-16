#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::punctuated::Punctuated;
use syn::{DeriveInput, Path, Token};

#[proc_macro_derive(TypeSlot, attributes(slot))]
pub fn derive_type_slot(input: TokenStream) -> TokenStream {
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
                    #[inline]
                    fn try_slot() -> Option<usize> {
                        __SLOT.get()
                    }

                    #[inline]
                    fn dyn_try_slot(&self) -> Option<usize> {
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

#[proc_macro_derive(SlotGroup)]
pub fn derive_slot_group(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    quote! {
        const _: () = {
            static __LEN: ::typeslot::AtomicSlot =
                ::typeslot::AtomicSlot::new();

            impl ::typeslot::SlotGroup for #name {
                #[inline]
                fn init() -> usize {
                    let len = ::typeslot::init_slot::<Self>();
                    __LEN.set(len);
                    len
                }

                #[inline]
                fn try_len() -> Option<usize> {
                    __LEN.get()
                }

                #[inline]
                fn len() -> usize {
                    __LEN.get().expect("group not initialized; call `SlotGroup::init` first")
                }
            }
        };
    }
    .into()
}
