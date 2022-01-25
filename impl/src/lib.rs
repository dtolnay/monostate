use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
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
    unimplemented!()
}

fn must_be_char(value: char) -> Result<TokenStream2> {
    unimplemented!()
}

fn must_be_int(lit: LitInt) -> Result<TokenStream2> {
    unimplemented!()
}

fn must_be_bool(value: bool) -> Result<TokenStream2> {
    unimplemented!()
}

fn unsupported(lit: Lit) -> Result<TokenStream2> {
    Err(Error::new(lit.span(), "unsupported monostate literal kind"))
}
