use darling::FromDeriveInput;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod event;
mod fungible_token;
mod nep141;
mod nep148;
mod owner;
mod pause;
mod rbac;
mod rename;

fn make_derive<T>(
    input: TokenStream,
    expand: fn(T) -> Result<TokenStream, syn::Error>,
) -> TokenStream
where
    T: FromDeriveInput,
{
    let input = parse_macro_input!(input as DeriveInput);
    let meta: T = match FromDeriveInput::from_derive_input(&input) {
        Err(e) => return e.write_errors().into(),
        Ok(x) => x,
    };

    expand(meta).unwrap_or_else(|e| e.into_compile_error().into())
}

/// Derives an NEP-297-compatible event emitting implementation of `Event`.
///
/// Specify event standard parameters: `#[event(standard = "...", version = "...")]`
///
/// Rename strategy for all variants (default: unchanged): `#[event(..., rename_all = "<strategy>")]`
/// Options for `<strategy>`:
/// - `UpperCamelCase`
/// - `lowerCamelCase`
/// - `snake_case`
/// - `kebab-case`
/// - `SHOUTY_SNAKE_CASE`
/// - `SHOUTY-KEBAB-CASE`
/// - `Title Case`
#[proc_macro_derive(Event, attributes(event))]
pub fn derive_event(input: TokenStream) -> TokenStream {
    make_derive(input, event::expand)
}

/// Creates a managed, lazily-loaded `Owner` implementation for the targeted
/// `#[near_bindgen]` struct.
///
/// The storage key prefix for the fields can be optionally specified (default:
/// `"~o"`) using `#[owner(storage_key = "<expression>")]`.
#[proc_macro_derive(Owner, attributes(owner))]
pub fn derive_owner(input: TokenStream) -> TokenStream {
    make_derive(input, owner::expand)
}

/// Makes a contract pausable. Provides an implementation of the `Pause` trait.
///
/// The storage key prefix for the fields can be optionally specified (default:
/// `"~p"`) using `#[pause(storage_key = "<expression>")]`.
#[proc_macro_derive(Pause, attributes(pause))]
pub fn derive_pause(input: TokenStream) -> TokenStream {
    make_derive(input, pause::expand)
}

/// Adds role-based access control. No external methods are exposed.
///
/// The storage key prefix for the fields can be optionally specified (default:
/// `"~r"`) using `#[rbac(storage_key = "<expression>")]`.
#[proc_macro_derive(Rbac, attributes(rbac))]
pub fn derive_rbac(input: TokenStream) -> TokenStream {
    make_derive(input, rbac::expand)
}

/// Adds NEP-141 fungible token core functionality to a contract. Exposes
/// `ft_*` functions to the public blockchain, implements internal controller
/// and receiver functionality (see: `near_contract_tools::standard::nep141`).
///
/// The storage key prefix for the fields can be optionally specified (default:
/// `"~$141"`) using `#[nep141(storage_key = "<expression>")]`.
#[proc_macro_derive(Nep141, attributes(nep141))]
pub fn derive_nep141(input: TokenStream) -> TokenStream {
    make_derive(input, nep141::expand)
}

/// Adds NEP-148 fungible token metadata functionality to a contract. Metadata
/// is hardcoded into the contract code, and is therefore not stored in storage.
/// 
/// Specify metadata using the `#[nep148(...)]` attribute.
/// 
/// Fields:
///  - `name`
///  - `symbol`
///  - `decimals`
///  - `spec` (optional)
///  - `icon` (optional)
///  - `reference` (optional)
///  - `reference_hash` (optional)
#[proc_macro_derive(Nep148, attributes(nep148))]
pub fn derive_nep148(input: TokenStream) -> TokenStream {
    make_derive(input, nep148::expand)
}

/// Implements NEP-141 and NEP-148 functionality, like
/// `#[derive(Nep141, Nep148)]`.
/// 
/// Attributes are the union of those for the constituent derive macros.
/// Specify attributes with `#[fungible_token(...)]`.
#[proc_macro_derive(FungibleToken, attributes(fungible_token))]
pub fn derive_fungible_token(input: TokenStream) -> TokenStream {
    make_derive(input, fungible_token::expand)
}
