use std::fmt::{Debug, Display, Formatter};

pub(crate) trait ToTokens {

    fn to_tokens(self) -> Result<Vec<TokenEntry>, String>;

}
impl ToTokens for String {
    fn to_tokens(self) -> Result<Vec<TokenEntry>, String> {
        let new_self = self.replace("from crunch_lib import *", "");
        let mut chars = new_self.chars().collect::<Vec<char>>();
        let mut tokens: Vec<TokenEntry> = Vec::new();
        if chars.len() == 0 { return Ok(tokens) }
        let mut maybe_cur: Option<char>;
        let mut next: Option<char> = Some(chars.remove(0));
        let collected: &mut Vec<char> = &mut Vec::new();
        let mut string_state = false;
        let mut ignore_text = false;
        let mut commenting = false;
        let mut line_number = 1;
        let mut space_count = 0;
        let mut counting_spaces = true;
        loop {
            maybe_cur = next;
            if chars.len() > 0 {
                next = Some(chars.remove(0));
            } else {
                next = None;
            }
            if maybe_cur == None { break }
            let cur = maybe_cur.unwrap();
            if cur == '\\' {
                if next == Some('"') || next == Some('\'') { ignore_text = true; }
            }
            if cur == '"' {
                if !ignore_text {
                    if !string_state {
                        string_state = true;
                        continue;
                    } else {
                        let finalized = collected.iter().collect::<String>();
                        tokens.push(TokenEntry { token: Token::StringLiteral(finalized), line_number });
                        collected.clear();
                        string_state = false;
                        continue;
                    }
                } else {
                    ignore_text = false;
                }
            }
            if string_state {
                collected.push(cur);
                continue;
            }
            if cur == '\r' || cur == '\n' {
                commenting = false;
            }
            if cur == '#' && next == Some('#') {commenting = true; }
            let token_ref = &mut tokens;
            if commenting {
                continue;
            }
            if cur == ' ' {
                space_count += 1;
            } else if cur != '\n' {
                if counting_spaces {
                    token_ref.push(TokenEntry { token : Token::Spaces(space_count), line_number });
                    space_count = 0;
                    counting_spaces = false;
                }
            } else {
                space_count = 0;
            }
            match cur {
                ':' => push_with_extra(Token::Colon, token_ref, collected, line_number)?,
                '[' => push_with_extra(match next {
                    Some(']') => {
                        next = Some(chars.remove(0));
                        Token::EmptyList
                    }
                    _ => { Token::OpenBracket }
                }, token_ref, collected, line_number)?,
                ']' => push_with_extra(Token::CloseBracket, token_ref, collected, line_number)?,
                '(' => push_with_extra(Token::OpenParenthesis, token_ref, collected, line_number)?,
                ')' => push_with_extra(Token::CloseParenthesis, token_ref, collected, line_number)?,
                ',' => push_with_extra(Token::Comma, token_ref, collected, line_number)?,
                '+' => push_with_extra(match next {
                    Some('=') => {
                        next = Some(chars.remove(0));
                        Token::PlusEquals
                    }
                    Some('+') => {
                        next = Some(chars.remove(0));
                        Token::Increment
                    }
                    _ => Token::Plus,
                }, token_ref, collected, line_number)?,
                '-' => push_with_extra(match next {
                    Some('-') => {
                        next = Some(chars.remove(0));
                        Token::Decrement
                    }
                    Some('=') => {
                        next = Some(chars.remove(0));
                        Token::MinusEquals
                    }
                    Some('>') => {
                        next = Some(chars.remove(0));
                        Token::RightArrow
                    }
                    _ => Token::Minus,
                }, token_ref, collected, line_number)?,
                '*' => push_with_extra(match next {
                    Some('=') => {
                        next = Some(chars.remove(0));
                        Token::MultiplyEquals
                    }
                    _ => { Token::Star }
                }, token_ref, collected, line_number)?,
                '/' => push_with_extra(match next {
                    Some('=') => {
                        next = Some(chars.remove(0));
                        Token::DivideEquals
                    }
                    _ => Token::Slash,
                }, token_ref, collected, line_number)?,
                '%' => push_with_extra(match next {
                    Some('=') => {
                        next = Some(chars.remove(0));
                        Token::PercentEquals
                    }
                    _ => Token::Percent,
                }, token_ref, collected, line_number)?,
                '<' => push_with_extra(match next {
                    Some('<') => {
                        next = Some(chars.remove(0));
                        Token::LeftShift
                    },
                    Some('=') => {
                        next = Some(chars.remove(0));
                        Token::LessThanEquals
                    },
                    _ => Token::LessThan,
                }, token_ref, collected, line_number)?,
                '>' => push_with_extra(match next {
                    Some('>') => {
                        next = Some(chars.remove(0));
                        Token::RightShift
                    },
                    Some('=') => {
                        next = Some(chars.remove(0));
                        Token::GreaterThanEquals
                    },
                    _ => Token::GreaterThan,
                }, token_ref, collected, line_number)?,
                '=' => push_with_extra(match next {
                    Some('=') => {
                        next = Some(chars.remove(0));
                        Token::EqualsEquals
                    }
                    _ => Token::Equals,
                }, token_ref, collected, line_number)?,
                '!' => push_with_extra(match next {
                    Some('=') => {
                        next = Some(chars.remove(0));
                        Token::NotEquals
                    }
                    _ => Token::Not,
                }, token_ref, collected, line_number)?,
                '&' => push_with_extra(match next {
                    Some('&') => {
                        next = Some(chars.remove(0));
                        Token::AndAnd
                    }
                    _ => Token::And,
                }, token_ref, collected, line_number)?,
                '|' => push_with_extra(match next {
                    Some('|') => {
                        next = Some(chars.remove(0));
                        Token::OrOr
                    }
                    _ => Token::Or,
                }, token_ref, collected, line_number)?,
                '\n' => {
                    line_number = line_number + 1;
                    push_collected(token_ref, collected, line_number)?;
                    counting_spaces = true;
                }
                ' ' | '\r' | '\t' => push_collected(token_ref, collected, line_number)?,
                _ => { collected.push(cur) }
            }
        }
        if !collected.is_empty() {
            push_collected(&mut tokens, collected, line_number)?;
        }
        loop {
            if tokens.len() < 2 { break; }
            if matches!(&tokens[0].token, Token::Spaces(_)) && matches!(&tokens[1].token, Token::Spaces(_)) {
                tokens.remove(0);
            } else { break; }
        }
        Ok(tokens)
    }
}

fn push_with_extra(token: Token, tokens: &mut Vec<TokenEntry>, collected: &mut Vec<char>, line_number: usize) -> Result<(), String> {
    push_collected(tokens, collected, line_number)?;
    tokens.push(TokenEntry { token, line_number });
    Ok(())
}

fn push_collected(tokens: &mut Vec<TokenEntry>, collected: &mut Vec<char>, line_number: usize) -> Result<(), String> {
    if collected.len() != 0 {
        let finalized = collected.iter().collect::<String>();
        collected.clear();
        let token = match finalized.as_str() {
            "return" => Token::Return,
            "if" => Token::If,
            "while" => Token::While,
            "else" => Token::Else,
            "for" => Token::For,
            "int" => Token::IntType,
            "str" => Token::StringType,
            "def" => Token::Def,
            "pass" => Token::Pass,
            "list" => Token::List,
            "Matrix" => Token::Matrix,
            "Complex" => Token::Complex,
            "del" => Token::Del,
            "float" => Token::Float,
            v => eval_literal(v.to_string())?
        };
        tokens.push( TokenEntry { token, line_number }
        );
    }
    Ok(())
}

fn eval_literal(finalized: String) -> Result<Token, String> {
    if let Ok(b) = finalized.parse::<bool>() {
        return Ok(Token::BoolLiteral(b));
    }
    if let Ok(i) = finalized.parse::<i64>() {
        return Ok(Token::IntLiteral(i));
    }
    if let Ok(f) = finalized.parse::<f64>() {
        return Ok(Token::FloatLiteral(f));
    }
    Ok(Token::Identifier(finalized))
}

#[derive(Clone)]
pub(crate) struct TokenEntry {
    pub(crate) token: Token,
    pub(crate) line_number: usize,
}

impl Display for TokenEntry {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}@{}", self.token, self.line_number))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Identifier(String),
    Colon,
    OpenBracket,
    CloseBracket,
    OpenParenthesis,
    CloseParenthesis,
    EmptyList,
    Comma,
    Plus,
    Minus,
    Increment,
    Decrement,
    Star,
    Slash,
    Percent,
    Equals,
    EqualsEquals,
    NotEquals,
    GreaterThan,
    GreaterThanEquals,
    LessThan,
    LessThanEquals,
    PlusEquals,
    MinusEquals,
    MultiplyEquals,
    DivideEquals,
    PercentEquals,
    LeftShift,
    RightShift,
    Not,
    AndAnd,
    And,
    OrOr,
    Or,
    StringLiteral(String),
    BoolLiteral(bool),
    IntLiteral(i64),
    FloatLiteral(f64),
    Return,
    If,
    While,
    Else,
    For,
    IntType,
    StringType,
    Def,
    Pass,
    Spaces(usize),
    List,
    Del,
    Float,
    EOF,
    Matrix,
    Complex,
    RightArrow,
}