use proc_macro::Span;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::punctuated::Punctuated;
use syn::FnArg;
use syn::Ident;
use syn::ItemStruct;
use syn::ItemTrait;
use syn::Meta;
use syn::TraitItem;

const SAHNE_MACRO_SUFFIX: &str = "SahneMixin";

#[proc_macro_attribute]
pub fn provider(attr: TokenStream, item: TokenStream) -> TokenStream {
    let user_trait_ident = parse_macro_input!(attr as Ident);
    let ast = item.clone();
    let ast: ItemTrait = parse_macro_input!(ast as ItemTrait);

    let original_trait_ident = &ast.ident;

    let macro_name = Ident::new(
        &format!("{}{}", original_trait_ident, SAHNE_MACRO_SUFFIX),
        Span::call_site().into(),
    );

    let macro_funs = ast.items.iter().flat_map(|x| {
        let TraitItem::Fn(f) = x else {
            return None;
        };

        let sig = &f.sig;
        let fn_name = &sig.ident;
        let fn_args = &sig
            .inputs
            .iter()
            .map(|x| match x {
                FnArg::Receiver(reciever) => {
                    let self_token = reciever.self_token;
                    quote! {
                    #self_token
                        }
                }
                FnArg::Typed(typed) => {
                    let pat = &typed.pat;
                    quote! {
                        #pat
                    }
                }
            })
            .collect::<Vec<_>>();
        Some(quote! {
            #sig {
        <Self as #original_trait_ident>::#fn_name(#(#fn_args),*)
            }
        })
    });

    quote! {
    #ast
    #[macro_export]
    macro_rules! #macro_name {
        ($user:ident) => {
        impl #original_trait_ident for $user {}
        impl #user_trait_ident for $user {
        #(#macro_funs)*
        }
        }
    }
    }
    .into()
}

#[proc_macro_attribute]
pub fn mixin(attr: TokenStream, item: TokenStream) -> TokenStream {
    let provider_trait_ident =
        parse_macro_input!(attr with Punctuated::<syn::Ident, syn::Token![,]>::parse_terminated);

    let ast = parse_macro_input!(item as ItemStruct);

    let target_struct_ident = &ast.ident;

    let macro_calls = provider_trait_ident.iter().map(|provider| {
        let macro_name = Ident::new(
            &format!("{}{}", provider, SAHNE_MACRO_SUFFIX),
            Span::call_site().into(),
        );
        quote! {
            #macro_name!(#target_struct_ident);
        }
    });

    quote! {
    #ast

    #(#macro_calls)*
    }
    .into()
}
