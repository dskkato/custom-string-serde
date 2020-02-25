extern crate proc_macro;
use proc_macro::TokenStream as StdTokenStream;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse::Parser, punctuated::Punctuated, Expr, Lit, Token};

#[proc_macro]
pub fn custom_string_serde(input: StdTokenStream) -> StdTokenStream {
    let tokens = input.clone();
    let parser = Punctuated::<Expr, Token![,]>::parse_terminated;
    let args = parser.parse(tokens).unwrap();
    let mut expanded = Vec::new();

    for arg in args {
        let lit = match arg {
            Expr::Lit(lit) => lit.lit,
            _ => continue,
        };
        let n = match lit {
            Lit::Int(lit_int) => lit_int,
            _ => continue,
        };
        let struct_name = format!("CustomString{}", n.base10_digits());
        let struct_ident = syn::Ident::new(&struct_name, n.span());
        let visitor_name = format!("CustomString{}Visitor", n.base10_digits());
        let visitor_ident = syn::Ident::new(&visitor_name, n.span());

        expanded.push(quote! {
           #[repr(transparent)]
           #[derive(Copy, Clone)]
           struct #struct_ident{
               bytes: [u8; #n],
           }

            impl std::fmt::Debug for #struct_ident {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    let n = self
                        .bytes
                        .iter()
                        .position(|&x| x == 0)
                        .unwrap_or(#n);
                    let s = std::str::from_utf8(&self.bytes[..n]).unwrap();

                    write!(f, "{}", s)
                }
            }

            impl std::cmp::PartialEq for #struct_ident {
                fn eq(&self, other: &Self) -> bool {
                    let n = self
                        .bytes
                        .iter()
                        .position(|&x| x == 0)
                        .unwrap_or(#n);
                    let s = std::str::from_utf8(&self.bytes[..n]).unwrap();

                    let n = other
                        .bytes
                        .iter()
                        .position(|&x| x == 0)
                        .unwrap_or(#n);
                    let o = std::str::from_utf8(&other.bytes[..n]).unwrap();

                    s == o
                }
            }

            impl Serialize for #struct_ident {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    let n = self
                        .bytes
                        .iter()
                        .position(|&x| x == 0)
                        .unwrap_or(#n);
                    let s = std::str::from_utf8(&self.bytes[..n]).unwrap();

                    serializer.serialize_str(s)
                }
            }

            impl<'de> Deserialize<'de> for #struct_ident {
                fn deserialize<D>(deserializer: D) -> Result<#struct_ident, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    use serde::de::{self, Visitor};
                    use std::fmt;

                    struct #visitor_ident;

                    impl<'de> Visitor<'de> for #visitor_ident {
                        type Value = #struct_ident;

                        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                            formatter.write_str("string is expected")
                        }

                        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                        where
                            E: de::Error,
                        {
                            let mut bytes = [0u8; #n];
                            bytes[..value.len()].copy_from_slice(&value.as_bytes()[..]);
                            Ok(Self::Value { bytes })
                        }

                        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
                        where
                            E: de::Error,
                        {
                            let mut bytes = [0u8; #n];
                            bytes[..value.len()].copy_from_slice(&value.as_bytes()[..]);
                            Ok(Self::Value { bytes })
                        }
                    }

                    deserializer.deserialize_string(#visitor_ident)
                }
            }
        });
    }

    expanded.into_iter().collect::<TokenStream>().into()
}
