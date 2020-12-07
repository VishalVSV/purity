#[derive(Debug)]
pub enum PureToken {
    Let,
    Colon,
    SemiColon,
    LParenthesis,
    RParenthesis,
    LBrace,
    RBrace,
    Func,
    Equals,
    Ident(String),
    StringLiteral(String),
    I32(i32)
}