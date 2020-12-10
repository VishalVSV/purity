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
    Arrow,

    Plus,
    Minus,
    Star,
    Divide,
    

    Ident(String),
    StringLiteral(String),
    I32(i32)
}