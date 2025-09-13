use crate::lexer::{Token, TokenEntry};
use crate::types::{Function, Parameter, Type};

pub(crate) fn parse_tokens(mut tokens: Vec<TokenEntry>) -> Result<Vec<Function> ,String> {
    while !tokens.is_empty() {
        let token = eat(&mut tokens);
        if !matches!(token.token, Token::Def) {
            return Err(format!("Expected 'def', but found {:?} on line {}", token.token, token.line_number));
        }
    }

    Ok(todo!())
}

fn parse_function(tokens : &mut Vec<TokenEntry>) -> Result<Function ,String> {
    let token = eat(tokens);
    let name = match token.token {
        Token::Identifier(name) => name,
        t => { return Err(format!("Expected function name, but found {:?} on line {}", t, token.line_number)) }
    };
    let expect = eat(tokens);
    match expect.token {
        Token::OpenParenthesis => {}
        t => { return Err(format!("Expected ')' after function name, but found {:?} on line {}", t, expect.line_number)) }
    }
    let mut parameters = Vec::new();
    if !matches!(peek(tokens), Token::CloseParenthesis) {
        parameters.push(parse_var_dec(tokens)?);
        loop {
            if matches!(peek(tokens), Token::CloseParenthesis) { break; }
            let expect = eat(tokens);
            match expect.token {
                Token::Comma => {}
                t => { return Err(format!("Expected ',' after function parameter, but found {:?} on line {}", t, expect.line_number)) }
            }
            parameters.push(parse_var_dec(tokens)?);
        }
    }
    let parameters = parameters.into_iter().map(|(name, typetype)| Parameter { name, typetype }).collect();
    let _ = eat(tokens);
    let return_type = if matches!(peek(tokens), Token::RightArrow) {
        let _ = eat(tokens);
        let typetype = parse_type(tokens)?;
        match typetype {
            Type::Int | Type::Float | Type::Complex => {}
            t => { return Err(format!("Expected return type of 'int', 'float' or 'Complex', but found {:?} on line {}", t, expect.line_number)) }
        }
        Some(typetype)
    } else {
        None
    };

    Ok(Function { name, parameters, return_type })
}

fn parse_var_dec(tokens : &mut Vec<TokenEntry>) -> Result<(String, Type), String> {
    let token = eat(tokens);
    let name = match token.token {
        Token::Identifier(name) => name,
        t => { return Err(format!("Expected variable name, but found {:?} on line {}", t, token.line_number)) }
    };
    let expect = eat(tokens);
    match expect.token {
        Token::Colon => {}
        t => { return Err(format!("Expected ':' after variable name, but found {:?} on line {}", t, expect.line_number)) }
    }
    Ok((name, parse_type(tokens)?))
}

fn parse_type(tokens : &mut Vec<TokenEntry>) -> Result<Type ,String> {
    let token = eat(tokens);
    Ok(match token.token {
        Token::IntType => Type::Int,
        Token::StringType => Type::String,
        Token::Float => Type::Float,
        Token::Matrix => Type::Matrix,
        Token::Complex => Type::Complex,
        Token::List => {
            let expect = eat(tokens);
            match expect.token {
                Token::OpenBracket => {}
                t => { return Err(format!("Expected '[' after list variable type, but found {:?} on line {}", t, expect.line_number)) }
            }
            let token = eat(tokens);
            let typetype = match token.token {
                Token::IntType => Type::IntList,
                Token::Complex => Type::ComplexList,
                Token::Float => Type::FloatList,
                t => { return Err(format!("Expected 'int', 'Complex', or 'float' for list generic, but found {:?} on line {}", t, expect.line_number)) }
            };
            let expect = eat(tokens);
            match expect.token {
                Token::CloseBracket => {}
                t => { return Err(format!("Expected ']' after list variable type, but found {:?} on line {}", t, expect.line_number)) }
            }
            typetype
        }
        t => { return Err(format!("Expected ':' after variable name, but found {:?} on line {}", t, token.line_number)) },
    })
}

fn eat(tokens: &mut Vec<TokenEntry>) -> TokenEntry {
    tokens.remove(0)
}

fn peek(tokens: &mut Vec<TokenEntry>) -> &Token {
    if tokens.len() == 0 { return &Token::EOF }
    &tokens.get(0).unwrap().token
}