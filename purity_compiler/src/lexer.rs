use crate::utils::PureToken;

use std::str::Chars;

pub struct Lexer<'a>
{
    chars: Chars<'a>,
    unused_char: Option<char>
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a String) -> Self {
        Self {
            chars: src.chars(),
            unused_char: None
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = PureToken;

    fn next(&mut self) -> Option<PureToken> {
        use PureToken::*;
        
        let mut c_init = 
        if let Some(c) = self.unused_char {
            self.unused_char = None;
            c
        }
        else {
            if let Some(chr) = self.chars.next() {
                chr
            }
            else {
                return None;
            }
        };

        while c_init.is_whitespace() {
            if let Some(chr) = self.chars.next() {
                c_init = chr;
            }
            else {
                return None;
            }
        }

        match c_init {
            '(' => Some(LParenthesis),
            ')' => Some(RParenthesis),
            ':' => Some(Colon),
            ';' => Some(SemiColon),
            '{' => Some(LBrace),
            '}' => Some(RBrace),
            '=' => Some(Equals),
            '"' => {
                let mut string = String::new();
                while let Some(c) = self.chars.next() {
                    if c != '"' {
                        string.push(c);
                    }
                    else {
                        break;
                    }
                }
                Some(StringLiteral(string))
            },
            c => {
                if c.is_alphabetic() || c == '_' {
                    let mut ident = String::new();
                    ident.push(c);
                    while let Some(c) = self.chars.next() {
                        if c.is_alphanumeric() || c == '_' {
                            ident.push(c);
                        }
                        else {
                            self.unused_char = Some(c);
                            break;
                        }
                    }
                    if ident.eq(&"fn") {
                        Some(Func)
                    }
                    else if ident.eq(&"let") {
                        Some(Let)
                    }
                    else {
                        Some(Ident(ident))
                    }
                }
                else if c.is_numeric() {
                    let mut num = String::new();
                    num.push(c);
                    while let Some(c) = self.chars.next() {
                        if c.is_numeric() {
                            num.push(c);
                        }
                        else {
                            self.unused_char = Some(c);
                            break;
                        }
                    }

                    Some(I32(num.parse().unwrap()))
                }
                else {
                    println!("Invalid char {}",c);
                    None
                }
            }
        }
    }
}