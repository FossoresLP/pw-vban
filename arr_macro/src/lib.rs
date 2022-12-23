extern crate proc_macro;
use proc_macro::{Delimiter, Group, TokenStream, TokenTree};
use quote::{quote, ToTokens};

use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Expr, LitInt, Token};

struct ArrayBuilder {
	items: Vec<Expand>,
}

impl Parse for ArrayBuilder {
	fn parse(input: ParseStream) -> Result<Self> {
		let mut items: Vec<Expand> = Vec::new();
		let item: Expand = input.parse()?;
		items.push(item);
		while input.parse::<Token![,]>().is_ok() {
			let item: Expand = input.parse()?;
			items.push(item);
		}
		Ok(ArrayBuilder { items: items })
	}
}

struct Expand {
	value: Expr,
	count: usize,
}

impl Parse for Expand {
	fn parse(input: ParseStream) -> Result<Self> {
		let value: Expr = input.parse()?;
		let count: usize;
		if input.parse::<Token![;]>().is_ok() {
			let lit: LitInt = input.parse()?;
			count = lit.base10_parse::<usize>()?;
		} else {
			count = 1;
		}
		Ok(Expand {
			value: value,
			count: count,
		})
	}
}

#[proc_macro]
pub fn arr(input: TokenStream) -> TokenStream {
	let builder = parse_macro_input!(input as ArrayBuilder);
	let mut stream = proc_macro2::TokenStream::new();
	for item in builder.items {
		for _ in 0..item.count {
			if !stream.is_empty() {
				stream.extend(quote!(,));
			}
			item.value.to_tokens(&mut stream);
		}
	}
	let group = TokenTree::Group(Group::new(Delimiter::Bracket, TokenStream::from(stream)));
	TokenStream::from(group)
}
