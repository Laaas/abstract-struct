#![feature(proc_macro)]

#![recursion_limit="128"]

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
//extern crate itertools;

#[macro_use]
extern crate quote;

use proc_macro2::TokenStream;
use syn::{ItemStruct, Ident, punctuated::Punctuated};

/// The main macro, check out the README for more information.
#[proc_macro_attribute]
pub fn abstract_struct(_: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input: ItemStruct = syn::parse(input).unwrap();

	let vis = &input.vis;

	let ident = &input.ident;
	let trait_ident = Ident::new(&format!("{}Abstract", ident), ident.span());

	let lifetimes: TokenStream = input.generics.lifetimes().flat_map(|l| quote!(#l,)).collect();
	let lifetime_arguments: TokenStream = input.generics.lifetimes().flat_map(|l| {
		let mut l = l.clone();
		l.colon_token = None;
		l.bounds = Punctuated::new();
		quote!(#l,)
	}).collect();

	let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
	assert!(where_clause.is_none(), "abstract_struct sadly does not support where clauses");

	let assoc_ty_decl: TokenStream = input.generics.type_params().flat_map(|p| {
		let mut p = p.clone();
		p.eq_token = None;
		p.default = None;
		quote!(type #p;)
	}).collect();

	let assoc_ty_args: TokenStream = input.generics.type_params().flat_map(|p| {
		let p_ident = &p.ident;
		quote!(<Self as #trait_ident<#lifetime_arguments>>::#p_ident,)
	}).collect();

	let assoc_ty_impl: TokenStream = input.generics.type_params().flat_map(|p| {
		let p_ident = &p.ident;
		quote!(type #p_ident = #p_ident;)
	}).collect();

	let expanded = quote! {
		#input

		impl #impl_generics From<abstract_struct::Wrapper<#ident #ty_generics>> for #ident #ty_generics {
		    fn from(w: abstract_struct::Wrapper<#ident #ty_generics>) -> #ident #ty_generics {
		        w.0
		    }
		}

		#[allow(dead_code)]
		impl #impl_generics #ident #ty_generics {
			fn wrap(self) -> abstract_struct::Wrapper<Self> {
				abstract_struct::Wrapper(self)
			}
		}

		#vis trait #trait_ident<#lifetimes> : std::ops::Deref<Target = #ident<#lifetime_arguments #assoc_ty_args>> + std::convert::Into<#ident<#lifetime_arguments #assoc_ty_args>> {
			#assoc_ty_decl
		}

		impl #impl_generics #trait_ident<#lifetime_arguments> for abstract_struct::Wrapper<#ident #ty_generics> {
			#assoc_ty_impl
		}
	};

	expanded.into()
}

/// This macro prints the result to stdout before giving it to rustc
#[proc_macro_attribute]
pub fn abstract_struct_debug(args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let output = abstract_struct(args, input);
	println!("result: {}", output);
	output
}
