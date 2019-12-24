extern crate proc_macro;
extern crate quote;

use proc_macro::{TokenStream, TokenTree};
use quote::__rs::TokenStream as QuoteStream;

// Token which holds both the value of the token and stream
// which can be used to write the token to final code
#[derive(Debug)]
pub struct Token<T> {
    pub value: T,
    pub stream: TokenStream,
}

// Enum representing the different types of tokens that will be
// contained in the attribute arguments
#[derive(Debug)]
pub enum TokenContainer {
    Int(Token<i64>),
    Equation(Token<Vec<TokenEnum>>),
    Identifier(Token<String>),
    Operation(Token<String>),
}

impl TokenContainer {
    fn get_stream(&self) -> TokenStream {
        match self {
            Token::Int(item) => item.stream.to_owned(),
            Token::Equation(item) => item.stream.to_owned(),
            Token::Identifier(item) => item.stream.to_owned(),
            Token::Operation(item) => item.stream.to_owned(),
        }
    }
}
