
use syn::{
    parse::{Parse, ParseStream},    
    token, Ident, Path, Token
};

use std::collections::{HashMap, HashSet};


mod keyword {
	syn::custom_keyword!(Block);
	syn::custom_keyword!(NodeBlock);
	syn::custom_keyword!(UncheckedExtrinsic);
	syn::custom_keyword!(Pallet);
	syn::custom_keyword!(Call);
	syn::custom_keyword!(Storage);
	syn::custom_keyword!(Event);
	syn::custom_keyword!(Config);
	syn::custom_keyword!(Origin);
	syn::custom_keyword!(Inherent);
	syn::custom_keyword!(ValidateUnsigned);
	syn::custom_keyword!(exclude_parts);
	syn::custom_keyword!(use_parts);
}

#[derive(Debug)]
pub enum RuntimeDeclaration {
    Implicit(ImplicitRuntimeDeclaration),
    Explicit(ExplicitRuntimeDeclaration),
}

#[derive(Debug)]
pub struct ImplicitRuntimeDeclaration {
    pub name: Ident,
    pub where_section: WhereSection,
    pub pallets: Vec<PalletDeclaration>,
}

#[derive(Debug)]
pub struct ExplicitRuntimeDeclaration {
    pub name: Ident,
    pub where_section: WhereSection,
    pub pallets: Vec<Pallet>,
    pub pallet_token: token::Brace,
}

impl Parse for RuntimeDeclaration {
	fn parse(input: ParseStream) -> Result<Self> {
		input.parse::<Token![pub]>()?;
		input.parse::<Token![enum]>()?;
		let name = input.parse::<syn::Ident>()?;
		let where_section = input.parse()?;
		let pallets =
			input.parse::<ext::Braces<ext::Punctuated<PalletDeclaration, Token![,]>>>()?;
		let pallets_token = pallets.token;

		match convert_pallets(pallets.content.inner.into_iter().collect())? {
			PalletsConversion::Implicit(pallets) =>
				Ok(RuntimeDeclaration::Implicit(ImplicitRuntimeDeclaration {
					name,
					where_section,
					pallets,
				})),
			PalletsConversion::Explicit(pallets) =>
				Ok(RuntimeDeclaration::Explicit(ExplicitRuntimeDeclaration {
					name,
					where_section,
					pallets,
					pallets_token,
				})),
		}
	}
}

/// The declaration of a pallet.
#[derive(Debug, Clone)]
pub struct PalletDeclaration {
	/// The name of the pallet, e.g.`System` in `System: frame_system`.
	pub name: Ident,
	/// Optional fixed index, e.g. `MyPallet ...  = 3,`.
	pub index: Option<u8>,
	/// The path of the pallet, e.g. `frame_system` in `System: frame_system`.
	pub path: PalletPath,
	/// The instance of the pallet, e.g. `Instance1` in `Council: pallet_collective::<Instance1>`.
	pub instance: Option<Ident>,
	/// The declared pallet parts,
	/// e.g. `Some([Pallet, Call])` for `System: system::{Pallet, Call}`
	/// or `None` for `System: system`.
	pub pallet_parts: Option<Vec<PalletPart>>,
	/// The specified parts, either use_parts or exclude_parts.
	pub specified_parts: SpecifiedParts,
}

#[derive(Debug)]
pub struct WhereSection {
    pub block: syn::TypePath,
    pub node_block: syn::TypePath,
    pub unchecked_extrinsic: syn::TypePath,
}

#[derive(Debug, Clone)]
pub struct PalletPath {
	pub inner: Path,
}

#[derive(Debug, Clone)]
pub enum SpecifiedParts {
	/// Use all the pallet parts except those specified.
	Exclude(Vec<PalletPartNoGeneric>),
	/// Use only the specified pallet parts.
	Use(Vec<PalletPartNoGeneric>),
	/// Use the all the pallet parts.
	All,
}

#[derive(Debug, Clone)]
pub struct PalletPart {
	pub keyword: PalletPartKeyword,
	pub generics: syn::Generics,
}

/// The declaration of a part without its generics
#[derive(Debug, Clone)]
pub struct PalletPartNoGeneric {
	keyword: PalletPartKeyword,
}

#[derive(Debug, Clone)]
pub enum PalletPartKeyword {
	Pallet(keyword::Pallet),
	Call(keyword::Call),
	Storage(keyword::Storage),
	Event(keyword::Event),
	Config(keyword::Config),
	Origin(keyword::Origin),
	Inherent(keyword::Inherent),
	ValidateUnsigned(keyword::ValidateUnsigned),
}

/// The final definition of a pallet with the resulting fixed index and explicit parts.
#[derive(Debug, Clone)]
pub struct Pallet {
	/// The name of the pallet, e.g.`System` in `System: frame_system`.
	pub name: Ident,
	/// Either automatically infered, or defined (e.g. `MyPallet ...  = 3,`).
	pub index: u8,
	/// The path of the pallet, e.g. `frame_system` in `System: frame_system`.
	pub path: PalletPath,
	/// The instance of the pallet, e.g. `Instance1` in `Council: pallet_collective::<Instance1>`.
	pub instance: Option<Ident>,
	/// The pallet parts to use for the pallet.
	pub pallet_parts: Vec<PalletPart>,
}

enum PalletsConversion {
	Implicit(Vec<PalletDeclaration>),
	Explicit(Vec<Pallet>),
}

fn convert_pallets(pallets: Vec<PalletDeclaration>) -> syn::Result<PalletsConversion> {
	if pallets.iter().any(|pallet| pallet.pallet_parts.is_none()) {
		return Ok(PalletsConversion::Implicit(pallets))
	}

	let mut indices = HashMap::new();
	let mut last_index: Option<u8> = None;
	let mut names = HashMap::new();

	let pallets = pallets
		.into_iter()
		.map(|pallet| {
			let final_index = match pallet.index {
				Some(i) => i,
				None => last_index.map_or(Some(0), |i| i.checked_add(1)).ok_or_else(|| {
					let msg = "Pallet index doesn't fit into u8, index is 256";
					syn::Error::new(pallet.name.span(), msg)
				})?,
			};

			last_index = Some(final_index);

			if let Some(used_pallet) = indices.insert(final_index, pallet.name.clone()) {
				let msg = format!(
					"Pallet indices are conflicting: Both pallets {} and {} are at index {}",
					used_pallet, pallet.name, final_index,
				);
				let mut err = syn::Error::new(used_pallet.span(), &msg);
				err.combine(syn::Error::new(pallet.name.span(), msg));
				return Err(err)
			}

			if let Some(used_pallet) = names.insert(pallet.name.clone(), pallet.name.span()) {
				let msg = "Two pallets with the same name!";

				let mut err = syn::Error::new(used_pallet, &msg);
				err.combine(syn::Error::new(pallet.name.span(), &msg));
				return Err(err)
			}

			let mut pallet_parts = pallet.pallet_parts.expect("Checked above");

			let available_parts =
				pallet_parts.iter().map(|part| part.keyword.name()).collect::<HashSet<_>>();

			// Check parts are correctly specified
			match &pallet.specified_parts {
				SpecifiedParts::Exclude(parts) | SpecifiedParts::Use(parts) =>
					for part in parts {
						if !available_parts.contains(part.keyword.name()) {
							let msg = format!(
								"Invalid pallet part specified, the pallet `{}` doesn't have the \
								`{}` part. Available parts are: {}.",
								pallet.name,
								part.keyword.name(),
								pallet_parts.iter().fold(String::new(), |fold, part| {
									if fold.is_empty() {
										format!("`{}`", part.keyword.name())
									} else {
										format!("{}, `{}`", fold, part.keyword.name())
									}
								})
							);
							return Err(syn::Error::new(part.keyword.span(), msg))
						}
					},
				SpecifiedParts::All => (),
			}

			// Set only specified parts.
			match pallet.specified_parts {
				SpecifiedParts::Exclude(excluded_parts) => pallet_parts.retain(|part| {
					!excluded_parts
						.iter()
						.any(|excluded_part| excluded_part.keyword.name() == part.keyword.name())
				}),
				SpecifiedParts::Use(used_parts) => pallet_parts.retain(|part| {
					used_parts.iter().any(|use_part| use_part.keyword.name() == part.keyword.name())
				}),
				SpecifiedParts::All => (),
			}

			Ok(Pallet {
				name: pallet.name,
				index: final_index,
				path: pallet.path,
				instance: pallet.instance,
				pallet_parts,
			})
		})
		.collect::<Result<Vec<_>>>()?;

	Ok(PalletsConversion::Explicit(pallets))
}