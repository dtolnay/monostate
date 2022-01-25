use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Error, Lit, LitInt, Result};

#[allow(non_snake_case)]
#[proc_macro]
pub fn MustBe(input: TokenStream) -> TokenStream {
    let lit = parse_macro_input!(input as Lit);
    let expanded = match lit {
        Lit::Str(lit) => must_be_str(lit.value()),
        Lit::Byte(lit) => must_be_byte(lit.value()),
        Lit::Char(lit) => must_be_char(lit.value()),
        Lit::Int(lit) => must_be_int(lit),
        Lit::Bool(lit) => must_be_bool(lit.value),
        Lit::ByteStr(_) | Lit::Float(_) | Lit::Verbatim(_) => unsupported(lit),
    };
    expanded.unwrap_or_else(Error::into_compile_error).into()
}

fn must_be_str(value: String) -> Result<TokenStream2> {
    unimplemented!()
}

fn must_be_byte(value: u8) -> Result<TokenStream2> {
    Ok(quote!(::monostate::MustBeU8::<#value>))
}

fn must_be_char(value: char) -> Result<TokenStream2> {
    Ok(quote!(::monostate::MustBeChar::<#value>))
}

fn must_be_int(lit: LitInt) -> Result<TokenStream2> {
    let token = lit.token();
    match lit.suffix() {
        "u8" => Ok(quote!(::monostate::MustBeU8::<#token>)),
        "u16" => Ok(quote!(::monostate::MustBeU16::<#token>)),
        "u32" => Ok(quote!(::monostate::MustBeU32::<#token>)),
        "u64" => Ok(quote!(::monostate::MustBeU64::<#token>)),
        "u128" => Ok(quote!(::monostate::MustBeU128::<#token>)),
        "i8" => Ok(quote!(::monostate::MustBeI8::<#token>)),
        "i16" => Ok(quote!(::monostate::MustBeI16::<#token>)),
        "i32" => Ok(quote!(::monostate::MustBeI32::<#token>)),
        "i64" => Ok(quote!(::monostate::MustBeI64::<#token>)),
        "i128" => Ok(quote!(::monostate::MustBeI128::<#token>)),
        "" => {
            if lit.base10_digits().starts_with('-') {
                Ok(quote!(::monostate::MustBeNegInt::<#token>))
            } else {
                Ok(quote!(::monostate::MustBePosInt::<#token>))
            }
        }
        suffix => {
            let msg = format!("unsupported integers suffix `{}`", suffix);
            Err(Error::new(lit.span(), msg))
        }
    }
}

fn must_be_bool(value: bool) -> Result<TokenStream2> {
    Ok(quote!(::monostate::MustBeBool::<#value>))
}

fn unsupported(lit: Lit) -> Result<TokenStream2> {
    Err(Error::new(lit.span(), "unsupported monostate literal kind"))
}
