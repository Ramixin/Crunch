use crate::lexer::{Token, TokenEntry};
use crate::statements::{parse_statement};
use crate::types::{Function, Parameter, Type};

pub(crate) fn parse_tokens(mut tokens: Vec<TokenEntry>) -> Result<Vec<Function> ,String> {
    let mut functions = Vec::new();
    while !tokens.is_empty() {
        let token = eat(&mut tokens);
        if let Token::Spaces(_) = token.token {
            let next = eat(&mut tokens);
            match next.token {
                Token::Def => functions.push(parse_function(&mut tokens)?),
                Token::NewLine => {}
                _ => {
                    return Err(format!("Expected 'def', but found {:?} on line {}", token.token, token.line_number));
                }
            }
        } else {
            return Err(format!("Expected 'def', but found {:?} on line {}", token.token, token.line_number));
        }
        match token.token {
            Token::Spaces(_) => {

                continue
            },
            _ => {

            }
        }
    }

    Ok(functions)
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
        t => { return Err(format!("Expected '(' after function name, but found {:?} on line {}", t, expect.line_number)) }
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
    let expect = eat(tokens);
    match expect.token {
        Token::Colon => {}
        t => { return Err(format!("Expected ':' after function parameters, but found {:?} on line {}", t, expect.line_number)) }
    }
    let expect = eat(tokens);
    match expect.token {
        Token::NewLine => {}
        t => { return Err(format!("Expected NewLine after function declaration, but found {:?} on line {}", t, expect.line_number)) }
    }
    let spacing_req = match peek(tokens) {
        Token::Spaces(v) => *v,
        t => { return Err(format!("Expected spaces before first statement of the function, but found {:?} on line {}", t, expect.line_number)) }
    };
    let mut statements = Vec::new();
    loop {
        match parse_statement(tokens, Some(spacing_req))? {
            Some(v) => statements.push(v),
            None => {}
        }
        match peek(tokens) {
            Token::Spaces(count) => {
                if *count < spacing_req { break; }
                else if *count == spacing_req {}
                else {
                    return Err(format!("found extra indentation on line {}", eat(tokens).line_number));
                }
            }
            Token::EOF => break,
            _ => {}
        }
    }

    Ok(Function { name, parameters, return_type, statements })
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

pub(crate) fn parse_type(tokens : &mut Vec<TokenEntry>) -> Result<Type ,String> {
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

 pub(crate) fn eat(tokens: &mut Vec<TokenEntry>) -> TokenEntry {
     if tokens.is_empty() {
         TokenEntry { token : Token::EOF, line_number: 0 }
     } else {
         tokens.remove(0)
     }

}

pub(crate) fn peek(tokens: &mut Vec<TokenEntry>) -> &Token {
    if tokens.len() == 0 { return &Token::EOF }
    &tokens.get(0).unwrap().token
}