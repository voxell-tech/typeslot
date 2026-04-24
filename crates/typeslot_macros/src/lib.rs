#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::parse_macro_input;
use syn::punctuated::Punctuated;
use syn::{DeriveInput, Path, Token, Type};
#[cfg(feature = "generic")]
use syn::{Generics, TypeParamBound, parse_quote};

fn default_typeslot_path() -> Path {
    syn::parse_str("::typeslot").unwrap()
}

fn parse_crate_override(input: ParseStream) -> syn::Result<Path> {
    if input.peek(Token![,]) {
        input.parse::<Token![,]>()?;
        if !input.is_empty() {
            input.parse::<Token![crate]>()?;
            input.parse::<Token![=]>()?;
            return input.parse::<Path>();
        }
    }
    Ok(default_typeslot_path())
}

/// ```ignore
/// register!(Group, Foo);
/// register!(Group, [Foo, Bar]);
/// register!(Group, FooGeneric<f32>);
/// ```
#[proc_macro]
pub fn register(input: TokenStream) -> TokenStream {
    let RegisterInput {
        group,
        targets,
        typeslot_path,
    } = parse_macro_input!(input as RegisterInput);

    let impls = targets.iter().map(|ty| {
        let ty: Type = syn::parse_quote!(#ty);
        register_one(&typeslot_path, &group, &ty)
    });

    quote! { #(#impls)* }.into()
}

fn register_one(
    typeslot_path: &Path,
    group: &Path,
    ty: &Type,
) -> TokenStream2 {
    quote! {
        const _: () = {
            static __SLOT: #typeslot_path::AtomicSlot = #typeslot_path::AtomicSlot::new();

            impl #typeslot_path::TypeSlot<#group> for #ty {
                #[inline]
                fn try_slot() -> ::core::option::Option<usize> {
                    __SLOT.get()
                }
                #[inline]
                fn dyn_try_slot(&self) -> ::core::option::Option<usize> {
                    __SLOT.get()
                }
            }

            #typeslot_path::inventory::submit! {
                #typeslot_path::TypeSlotEntry {
                    type_id: ::core::any::TypeId::of::<#ty>(),
                    group_id: ::core::any::TypeId::of::<#group>(),
                    slot: &__SLOT,
                }
            }
        };
    }
}

/// Input structure for [`register`].
struct RegisterInput {
    group: Path,
    targets: Vec<Path>,
    typeslot_path: Path,
}

impl Parse for RegisterInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let group = input.parse::<Path>()?;
        input.parse::<Token![,]>()?;

        let targets: Vec<Path> = if input.peek(syn::token::Bracket) {
            let content;
            syn::bracketed!(content in input);
            Punctuated::<Path, Token![,]>::parse_terminated(&content)?
                .into_iter()
                .collect()
        } else {
            vec![input.parse::<Path>()?]
        };

        let typeslot_path = parse_crate_override(input)?;
        Ok(Self {
            group,
            targets,
            typeslot_path,
        })
    }
}

/// ```ignore
/// register_generic!(Group, Foo<T: Default, U>);
/// register_generic!(Group, [Foo<T: Default>, Bar<T>]);
/// ```
#[cfg(feature = "generic")]
#[proc_macro]
pub fn register_generic(input: TokenStream) -> TokenStream {
    let RegisterGenericInput {
        group,
        targets,
        typeslot_path,
    } = parse_macro_input!(input as RegisterGenericInput);

    let impls =
        targets.iter().map(|PathGenerics { path, generics }| {
            let (impl_generics, ty_generics, where_clause) =
                generics.split_for_impl();

            let ty = quote! { #path #ty_generics };
            register_generic_one(
                &typeslot_path,
                &impl_generics,
                &group,
                &ty,
                &where_clause,
            )
        });

    quote! { #(#impls)* }.into()
}

#[cfg(feature = "generic")]
fn register_generic_one(
    typeslot_path: &Path,
    impl_generics: &impl quote::ToTokens,
    group: &Path,
    ty: &impl quote::ToTokens,
    where_clause: &impl quote::ToTokens,
) -> TokenStream2 {
    quote! {
        impl #impl_generics #typeslot_path::TypeSlot<#group> for #ty #where_clause {
            #[inline]
            fn try_slot() -> ::core::option::Option<usize> {
                <#group as #typeslot_path::SlotGroupGen>::__try_generic_slot(
                    ::core::any::TypeId::of::<Self>()
                )
            }
            #[inline]
            fn dyn_try_slot(&self) -> ::core::option::Option<usize> {
                <#group as #typeslot_path::SlotGroupGen>::__try_generic_slot(
                    ::core::any::TypeId::of::<Self>()
                )
            }
        }
    }
}

/// Input structure for [`register_generic`].
#[cfg(feature = "generic")]
struct RegisterGenericInput {
    group: Path,
    targets: Vec<PathGenerics>,
    typeslot_path: Path,
}

#[cfg(feature = "generic")]
impl Parse for RegisterGenericInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let group = input.parse::<Path>()?;
        input.parse::<Token![,]>()?;

        let targets = if input.peek(syn::token::Bracket) {
            let content;
            syn::bracketed!(content in input);
            Punctuated::<PathGenerics, Token![,]>::parse_terminated(
                &content,
            )?
            .into_iter()
            .collect()
        } else {
            vec![input.parse::<PathGenerics>()?]
        };

        let typeslot_path = parse_crate_override(input)?;
        Ok(Self {
            group,
            targets,
            typeslot_path,
        })
    }
}

#[cfg(feature = "generic")]
struct PathGenerics {
    path: Path,
    generics: Generics,
}

#[cfg(feature = "generic")]
impl Parse for PathGenerics {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path = input.call(Path::parse_mod_style)?;
        let mut generics: Generics = input.parse()?;
        for param in generics.type_params_mut() {
            let already_static = param.bounds.iter().any(|b| {
                matches!(b, TypeParamBound::Lifetime(l) if l.ident == "static")
            });
            if !already_static {
                param.bounds.push(parse_quote!('static));
            }
        }
        Ok(Self { path, generics })
    }
}

// #[derive(TypeSlot)]

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

    let typeslot_path = default_typeslot_path();

    if input.generics.params.is_empty() {
        let impls = groups.iter().map(|group| {
            let ty: Type = syn::parse_quote!(#name);
            register_one(&typeslot_path, group, &ty)
        });
        return quote! { #(#impls)* }.into();
    }

    #[cfg(not(feature = "generic"))]
    return syn::Error::new_spanned(
        &input.generics,
        "enable the `generic` feature of `typeslot` to use \
        `#[derive(TypeSlot)]` on generic types; alternatively, \
        register manually with `register_generic!`",
    )
    .to_compile_error()
    .into();

    #[cfg(feature = "generic")]
    {
        let mut generics = input.generics.clone();
        for param in generics.type_params_mut() {
            param.bounds.push(parse_quote!('static));
        }
        let (impl_generics, ty_generics, where_clause) =
            generics.split_for_impl();

        let impls = groups.iter().map(|group| {
            let ty = quote::quote!(#name #ty_generics);
            register_generic_one(
                &typeslot_path,
                &impl_generics,
                group,
                &ty,
                &where_clause,
            )
        });

        quote! { #(#impls)* }.into()
    }
}

/// ```ignore
/// #[derive(SlotGroup)]
/// // Add this attribute to support `HashMap` based slot indices
/// // for generics. Only available if `generic` feature is enabled.
/// #[generic]
/// struct Group;
/// ```
#[proc_macro_derive(SlotGroup, attributes(generic))]
pub fn derive_slot_group(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    #[cfg_attr(not(feature = "generic"), expect(unused_mut))]
    let mut try_len_body = quote!(__SLOT_LEN.get());
    #[cfg_attr(not(feature = "generic"), expect(unused_mut))]
    let mut len_body = quote! {
        #try_len_body.expect("group not initialized; call `SlotGroup::init` first")
    };

    #[cfg_attr(not(feature = "generic"), expect(unused_mut))]
    let mut slot_group_gen_impl = quote!();

    #[cfg(feature = "generic")]
    {
        let generic_compatible = input
            .attrs
            .iter()
            .find(|a| a.path().is_ident("generic"))
            .is_some();

        if generic_compatible {
            slot_group_gen_impl = quote! {
                static __GENERIC_LEN: ::core::sync::atomic::AtomicUsize =
                    ::core::sync::atomic::AtomicUsize::new(0);
                static __GENERIC_MAP: ::typeslot::spin::Lazy<
                    ::typeslot::spin::RwLock<
                        ::typeslot::hashbrown::HashMap<::core::any::TypeId, usize>
                    >
                > = ::typeslot::spin::Lazy::new(|| {
                    ::typeslot::spin::RwLock::new(::typeslot::hashbrown::HashMap::new())
                });

                impl ::typeslot::SlotGroupGen for #name {
                    #[inline]
                    fn __try_generic_slot(
                        type_id: ::core::any::TypeId,
                    ) -> ::core::option::Option<usize> {
                        {
                            let guard = __GENERIC_MAP.read();
                            if let Some(&slot) = guard.get(&type_id) {
                                return Some(slot);
                            }
                        }
                        let base = __SLOT_LEN.get()?;
                        let mut guard = __GENERIC_MAP.write();
                        Some(*guard.entry(type_id).or_insert_with(|| {
                            let idx = __GENERIC_LEN.fetch_add(
                                1,
                                ::core::sync::atomic::Ordering::Relaxed,
                            );
                            base + idx
                        }))
                    }
                }
            };

            try_len_body = quote! {
                #try_len_body.map(|n| {
                    n + __GENERIC_LEN.load(::core::sync::atomic::Ordering::Relaxed)
                })
            };

            len_body = quote! {
                #len_body + __GENERIC_LEN.load(::core::sync::atomic::Ordering::Relaxed)
            };
        }
    }

    quote! {
        const _: () = {
            static __SLOT_LEN: ::typeslot::AtomicSlot = ::typeslot::AtomicSlot::new();

            impl ::typeslot::SlotGroup for #name {
                #[inline]
                fn init() -> usize {
                    let len = ::typeslot::init_slot::<Self>();
                    __SLOT_LEN.set(len);
                    len
                }

                #[inline]
                fn try_len() -> Option<usize> {
                    #try_len_body
                }

                #[inline]
                fn len() -> usize {
                    #len_body
                }
            }

            #slot_group_gen_impl
        };
    }
    .into()
}
