use crate::token::Token;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    //  标识符 + 字面量
    IDENT, // add，foobar, x, y, z,...
    INT,   // 12345

    // 运算符
    ASSIGN,
    PLUS,

    // 分隔符
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // 关键字
    FUNCTION,
    LET,
}

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut m = HashMap::new();
        m.insert("fn", TokenType::FUNCTION);
        m.insert("let", TokenType::LET);
        m
    };
}

pub fn lookup_ident(ident: &str) -> TokenType {
    match KEYWORDS.get(ident) {
        Some(value) => value.clone(),
        None => TokenType::IDENT,
    }
}
