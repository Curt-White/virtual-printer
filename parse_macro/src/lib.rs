extern crate proc_macro;
extern crate quote;
extern crate inflector;

use inflector::Inflector;
use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use syn::*;

#[derive(Debug)]
struct TokenType<T> {
    pub value: T,
    pub stream: TokenStream,
}

#[derive(Debug)]
enum Token {
    Int(TokenType<i64>),
    Equation(TokenType<Vec<Token>>),
    Identifier(TokenType<String>),
    Operation(TokenType<String>),
}

impl Token {
    fn get_stream(&self) -> TokenStream {
        match self {
            Token::Int(item) => item.stream.to_owned(),
            Token::Equation(item) => item.stream.to_owned(),
            Token::Identifier(item) => item.stream.to_owned(),
            Token::Operation(item) => item.stream.to_owned(),
        }
    }
}

// Creates a token out of the provided input which can be either an Integer,
// Equation, or a valid Rust identifier.
fn create_token(tokens: &Vec<TokenTree>) -> Token {
    let token = if tokens.len() == 1 {
        match tokens[0].to_owned() {
            TokenTree::Ident(ident) => Token::Identifier(TokenType {
                value: ident.to_string(),
                stream: TokenStream::from(TokenTree::from(ident)),
            }),
            TokenTree::Literal(lit) => {
                if lit.to_string().parse::<i64>().is_ok() {
                    Token::Int(TokenType {
                        value: lit.to_string().parse().unwrap(),
                        stream: TokenStream::from(TokenTree::from(lit)),
                    })
                } else {
                    panic!("The literal '{}' is not an integer", lit.to_string());
                }
            }
            TokenTree::Punct(punct) => Token::Operation(TokenType {
                value: punct.to_string(),
                stream: TokenStream::from(TokenTree::from(punct)),
            }),
            _ => panic!(
                "{} is not a Valid Identifier or Literal",
                tokens[0].to_string()
            ),
        }
    } else {
        let stream: Vec<Token> = tokens
            .into_iter()
            .map(|token| create_token(&vec![token.to_owned()]))
            .collect();
        let mut new_stream = TokenStream::new();
        new_stream.extend(tokens.to_owned());
        Token::Equation(TokenType {
            value: stream,
            stream: new_stream,
        })
    };

    token
}

// Parse the attributes into valid token that will be used to specify the
// lengths of each of the tokens being parsed from the byte stream.
fn parse_attrs(metadata: TokenStream) -> Vec<Token> {
    let mut final_tokens: Vec<Token> = Vec::new();
    let mut meta_iter = metadata.into_iter().peekable();
    let mut buffer: Vec<TokenTree> = Vec::new();

    loop {
        match meta_iter.next() {
            Some(item) => match item.to_string().as_ref() {
                "," => (),
                _ => buffer.push(item),
            },
            None => return final_tokens,
        };

        let next = meta_iter.peek();
        if next.is_none() || next.unwrap().to_string() == ",".to_owned() {
            let tokens = create_token(&buffer);
            final_tokens.push(tokens);
            buffer = Vec::new();
        }
    }
}

// Generate the function code for parsing the data from a byte stream
fn fn_content(
    name: &Ident,
    members: Vec<&Ident>,
    types: Vec<&Type>,
    tokens: Vec<Token>,
) -> quote::__rt::TokenStream {
    if tokens.len() != members.len() && tokens.len() != types.len() {
        panic!("The number of attr args and struct members must be the same");
    }

    let mut equations: Vec<quote::__rt::TokenStream> = Vec::new();
    let member2: Vec<Ident> = members
        .to_owned()
        .into_iter()
        .map(|item| item.to_owned())
        .collect();

    for idx in 0..=members.len() - 1 {
        let ident = members[idx].to_owned();
        let ty = types[idx].to_owned();
        let tok = tokens[idx].get_stream();
        let parsed = quote::__rt::TokenStream::from(tok);
        let ty_string = quote!(#ty).to_string();

        let data;
        if ty_string.contains("Vec") {
            data = quote! {
                let #ident = bytes.drain(..((#parsed) as usize)).collect();
            };
        } else {
            data = quote! {
                let #ident = bytes.remove(0);
            };
        }
        equations.push(data);
    }

    let code = quote! {
        #(#equations)*
        Ok(#name { #(#member2: #member2),* })
    };

    code
}

#[proc_macro_attribute]
pub fn gen_lex(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    // validate_attrs(metadata);
    let value = parse_attrs(metadata);

    let struct_data = match &input.data {
        Data::Struct(data) => &data.fields,
        _ => panic!("This macro may only be applied to a struct type"),
    };

    let mut idents: Vec<&Ident> = Vec::new();
    let mut types: Vec<&Type> = Vec::new();
    for item in struct_data {
        types.push(&item.ty);
        match &item.ident {
            Some(ident) => idents.push(ident),
            None => continue,
        };
    }

    let name = &input.ident;
    let func_name = format!("parse_{}", name).to_snake_case();
    let func_ident = syn::Ident::new(func_name.as_str(), name.span());

    let content = fn_content(name, idents, types, value);

    let expanded = quote! {
        #input

        fn #func_ident (bytes: &mut Vec<u8>) -> Result<#name, std::io::Error> {
            #content
        }
    };

    TokenStream::from(expanded)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
