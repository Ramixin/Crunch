use crate::lexer::{Token, TokenEntry};
use std::fmt::{Display, Formatter};
use crate::parser;
use crate::parser::{eat, peek};
use crate::types::Type;

// Thank you, Core Dumped, for the amazing video on Pratt Parsing!
pub(crate) fn parse_statement(tokens : &mut Vec<TokenEntry>, spacing_req : Option<usize>) -> Result<Option<Statement>, String> {
    println!("parsing statement");
    print!("[ ");
    for token in tokens.iter() {
        print!("{} ", token);
    }
    println!("]");
    if let Some(req) = spacing_req {
        let token = eat(tokens);
        match token.token {
            Token::Spaces(val) => {
                if val != req {
                    return Err(format!("Found suspicious indentation on line {}", token.line_number));
                }
            }
            t => {
                return Err(format!("Expected spacing token, but found {:?} on line {}", t, token.line_number));
            }
        }
    }
    let token = eat(tokens);
    let val = Ok(Some(match token.token {
        Token::NewLine => {
            return Ok(None)
        }
        Token::If => {
            let condition = parse_expression(tokens, 0)?;
            let body = parse_statement_body(tokens)?;
            let else_body = if matches!(peek(tokens), Token::Else) {
                let else_token = eat(tokens);
                if matches!(peek(tokens), Token::If) {
                    parse_statement(tokens, None)?
                } else {
                    Some(Statement::new(StatementType::If(Expression::new(ExpressionType::BoolLiteral(true), else_token.line_number), parse_statement_body(tokens)?, None), else_token.line_number))
                }
            } else {
                None
            };
            Statement::new(StatementType::If(condition, body, else_body), token.line_number)
        }
        Token::While => {
            let condition = parse_expression(tokens, 0)?;
            let body = parse_statement_body(tokens)?;
            Statement::new(StatementType::While(condition, body), token.line_number)
        }
        Token::For => {
            let token = eat(tokens);
            let loop_var = match token.token {
                Token::Identifier(name) => name,
                t => return Err(format!("Expected variable name after 'for', but found {:?} on line {}", t, token.line_number))
            };
            let token = eat(tokens);
            match token.token {
                Token::In => {},
                t => return Err(format!("Expected 'in' after for-loop variable name, but found {:?} on line {}", t, token.line_number))
            };
            let token = eat(tokens);
            let looped_var = match token.token {
                Token::Identifier(name) => name,
                t => return Err(format!("Expected list variable name after 'in', but found {:?} on line {}", t, token.line_number))
            };
            Statement::new(StatementType::For(loop_var, looped_var, parse_statement_body(tokens)?), token.line_number)
        }
        Token::Return => {
            let value =
            if matches!(peek(tokens), Token::NewLine) {
                eat(tokens);
                None
            } else {
                Some(parse_expression(tokens, 0)?)
            };
            Statement::new(StatementType::Return(value), token.line_number)
        }
        t  => {
            tokens.insert(0, TokenEntry { token : t, line_number : token.line_number });
            let expression = parse_expression(tokens, 0)?;
            let next = eat(tokens);
            new_line(
                match next.token {
                    Token::NewLine | Token::EOF => {
                        tokens.insert(0, next);
                        Statement::new(StatementType::Ignored(expression), token.line_number)
                    },
                    Token::Colon => {
                        let type_ = parser::parse_type(tokens)?;
                        let expect = eat(tokens);
                        match expect.token {
                            Token::Equals => {}
                            _ => { return Err(format!("expected '=', but found {:?} on line {}", expect.token, expect.line_number)) }
                        }
                        let expr = parse_expression(tokens, 0)?;
                        Statement::new(StatementType::Declaration(expression, type_, expr), token.line_number)
                    }
                    t @ (Token::Equals |
                    Token::PlusEquals |
                    Token::MinusEquals |
                    Token::MultiplyEquals |
                    Token::DivideEquals |
                    Token::PercentEquals) => {
                        let expr = parse_expression(tokens, 0)?;
                        let expr = {
                            if matches!(t, Token::Equals) {
                                expr
                            } else {
                                Expression::new(ExpressionType::Operation(match t {
                                    Token::PlusEquals => Operation::Add(expression.clone(), expr),
                                    Token::MinusEquals => Operation::Sub(expression.clone(), expr),
                                    Token::MultiplyEquals => Operation::Mul(expression.clone(), expr),
                                    Token::DivideEquals => Operation::Div(expression.clone(), expr),
                                    Token::PercentEquals => Operation::Mod(expression.clone(), expr),
                                    _ => unreachable!()
                                }), next.line_number)
                            }
                        };
                        Statement::new(StatementType::Assignment(expression, expr), token.line_number)
                    }
                    _ => { return Err(format!("unexpected token {:?} on line {}", next.token, next.line_number)) }
                }, tokens
            )?
        }
    }));
    val
}

fn new_line<A>(val : A, tokens: &mut Vec<TokenEntry>) -> Result<A, String> {
    let token = eat(tokens);
    if !matches!(token.token, Token::NewLine | Token::EOF) { Err(format!("expected New Line on line {}", token.line_number)) }
    else { Ok(val) }
}

pub(crate) fn parse_statement_body(tokens: &mut Vec<TokenEntry>) -> Result<Vec<Statement>, String> {
    let expected = eat(tokens);
    if !matches!(expected.token, Token::Colon) { return Err(format!("expected ':' but found {:?} on line {}", expected.token, expected.line_number)); }
    let expected = eat(tokens);
    if !matches!(expected.token, Token::NewLine) { return Err(format!("expected New Line but found {:?} on line {}", expected.token, expected.line_number)); }
    let spacing_req = if let Token::Spaces(count) = peek(tokens) {
        *count
    } else {
        let expected = eat(tokens);
        return Err(format!("expected indentation but found {:?} on line {}", expected.token, expected.line_number));
    };
    let mut statements: Vec<Statement> = Vec::new();
    loop {
        let statement = parse_statement(tokens, Some(spacing_req))?;
        match statement {
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
            Token::EOF => { break; }
            _ => {}
        }
    }
    Ok(statements)
}

pub(crate) fn parse_expression(tokens : &mut Vec<TokenEntry>, min_binding : u8) -> Result<Expression, String> {
    if matches!(peek(tokens), Token::NewLine) {
        return Ok(Expression::new(ExpressionType::Empty, eat(tokens).line_number))
    }
    let mut first = parse_side(tokens)?;
    println!("finished first");
    loop {
        match peek(tokens) {
            Token::NewLine | Token::CloseParenthesis | Token::EOF => break,
            _ => {}
        }
        println!("finding infix");
        if !valid_infix_operator(peek(tokens)) {
            return Ok(first);
        }
        let operator_token = eat(tokens);
        let operator = create_infix_operator::<()>(operator_token.clone())?;
        let (precedence, polarity) = get_binding(&operator_token)?;
        if precedence < min_binding || (precedence == min_binding && matches!(polarity, Polarity::LEFT)) {
            break;
        }
        let second = parse_side(tokens)?;

        first = operator(first, second);
    }
    Ok(first)
}

enum Either<T, R> {
    Left(T),
    Right(R),
}

fn parse_side(tokens : &mut Vec<TokenEntry>) -> Result<Expression, String> {
    println!("parsing side with first token: {}", &tokens[0]);
    let prefix = match create_prefix_operator(eat(tokens)) {
        Either::Left(v) => Some(v),
        Either::Right(r) => { tokens.insert(0, r); None }
    };
    let side = eat(tokens);
    let side = match side.token {
        Token::FloatLiteral(f) => Expression::new(ExpressionType::FloatLiteral(f), side.line_number),
        Token::IntLiteral(i) => Expression::new(ExpressionType::IntLiteral(i), side.line_number),
        Token::StringLiteral(s) => Expression::new(ExpressionType::StringLiteral(s), side.line_number),
        Token::BoolLiteral(b) => Expression::new(ExpressionType::BoolLiteral(b), side.line_number),
        Token::Identifier(i) => {
            if !matches!(peek(tokens), Token::OpenParenthesis) {
                Expression::new(ExpressionType::Field(i), side.line_number)
            } else {
                let args = parse_call_args(tokens)?;
                Expression::new(ExpressionType::Call(i, args), side.line_number)
            }
        }
        Token::OpenParenthesis => {
            let value = parse_expression(tokens, 0)?;
            let token = eat(tokens);
            match token.token {
                Token::CloseParenthesis => value,
                _ => { return Err(format!("expected ')' on line {}", token.line_number)) }
            }
        }
        Token::EmptyList => {
            Expression::new(ExpressionType::ListLiteral(Vec::new()), side.line_number)
        }
        Token::OpenBracket => {
            let mut literals = Vec::new();
            loop {
                literals.push(parse_expression(tokens, 0)?);
                let token = eat(tokens);
                match token.token {
                    Token::CloseBracket => {
                        break;
                    }
                    Token::Comma => {}
                    _ => {
                        return Err(format!("Expected ',' or ']' in list literal, but found {:?} on line {}", side.token, side.line_number))
                    }
                }
            }
            Expression::new(ExpressionType::ListLiteral(literals), side.line_number)
        }
        _ => { return Err(format!("Unexpected token: {:?} on line {}", side.token, side.line_number)) }
    };
    println!("side: {:?}", side);
    print!("[ ");
    for token in tokens.iter() {
        print!("{} ", token);
    }
    println!("]");
    let side = match prefix {
        None => side,
        Some(entry) => {
            entry(side)
        }
    };
    let postfix = match create_postfix_operator(eat(tokens)) {
        Either::Left(v) => Some(v),
        Either::Right(r) => { tokens.insert(0, r); None }
    };
    if let Some(_) = postfix {
        println!("has postfix");
    } else {
        println!("has no postfix");
    }
    Ok(if let Some(postfix) = postfix {
        match postfix {
            Either::Left(normal_postfix) => normal_postfix(side),
            Either::Right(array_postfix) => {
                let expr = parse_expression(tokens, 0)?;
                let expect = eat(tokens);
                match expect.token {
                    Token::CloseBracket => {},
                    t => { return Err(format!("expected ']', but found {:?} on line {}", t, expect.line_number)) }
                }
                array_postfix(expr, side)
            },
        }
    } else {
        println!("ending side parsing");
        side
    })
}



fn get_binding(entry : &TokenEntry) -> Result<(u8, Polarity), String> {
    Ok(match entry.token {
        Token::Plus => (6, Polarity::LEFT),
        Token::Minus => (6, Polarity::LEFT),
        Token::Star => (5, Polarity::LEFT),
        Token::Slash => (5, Polarity::LEFT),
        Token::Percent => (5, Polarity::LEFT),
        Token::Equals => (17, Polarity::RIGHT),
        Token::EqualsEquals => (10, Polarity::LEFT),
        Token::NotEquals => (10, Polarity::LEFT),
        Token::GreaterThan => (9, Polarity::LEFT),
        Token::GreaterThanEquals => (9, Polarity::LEFT),
        Token::LessThan => (8, Polarity::LEFT),
        Token::LessThanEquals => (8, Polarity::LEFT),
        Token::PlusEquals => (18, Polarity::RIGHT),
        Token::MinusEquals => (18, Polarity::RIGHT),
        Token::MultiplyEquals => (19, Polarity::RIGHT),
        Token::DivideEquals => (19, Polarity::RIGHT),
        Token::PercentEquals => (20, Polarity::RIGHT),
        Token::LeftShift => (7, Polarity::LEFT),
        Token::RightShift => (7, Polarity::LEFT),
        Token::AndAnd => (14, Polarity::LEFT),
        Token::And => (11, Polarity::LEFT),
        Token::OrOr => (15, Polarity::LEFT),
        Token::Or => (13, Polarity::LEFT),
        _ => return Err(format!("failed to get operator precedence of token '{:?}' on line {}", entry.token, entry.line_number))
    })
}

fn parse_call_args(tokens : &mut Vec<TokenEntry>) -> Result<Vec<Expression>, String> {
    let mut parameters = Vec::new();
    let token = eat(tokens);
    if !matches!(token.token, Token::OpenParenthesis) { return Err(format!("expected '(' on line {:?}", token.line_number)); }
    if matches!(peek(tokens), Token::CloseParenthesis) {
        eat(tokens);
        return Ok(parameters);
    }
    loop {
        parameters.push(parse_expression(tokens, 0)?);
        let token = eat(tokens);
        match token.token {
            Token::CloseParenthesis => break,
            Token::Comma => {},
            _ => { return Err(format!("expected ')' or ',' on line {}", token.line_number)); }
        }
    }
    Ok(parameters)
}

enum Polarity {
    LEFT,
    RIGHT
}

#[derive(Debug, Clone)]
pub struct Expression {
    pub(crate) type_ : Box<ExpressionType>,
    pub(crate) line_number : usize
}

impl Expression {

    pub fn new(type_ : ExpressionType, line_number : usize) -> Expression {
        Expression { type_: Box::from(type_), line_number }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum Operation {
    Construction(Vec<Expression>),
    Add(Expression, Expression),
    Sub(Expression, Expression),
    Mul(Expression, Expression),
    Div(Expression, Expression),
    Mod(Expression, Expression),
    Equals(Expression, Expression),
    GreaterThan(Expression, Expression),
    LessThan(Expression, Expression),
    GreaterEquals(Expression, Expression),
    LessEquals(Expression, Expression),
    LBS(Expression, Expression),
    RBS(Expression, Expression),
    ArrayIndex(Expression, Expression),
    BinaryAnd(Expression, Expression),
    BinaryOr(Expression, Expression),
    And(Expression, Expression),
    Or(Expression, Expression),
    Not(Expression),
    Negative(Expression),
    NotEqual(Expression, Expression),
    Increment(Expression),
    Decrement(Expression),
    Optional(Expression)
}

#[derive(Debug)]
pub(crate) enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Equals,
    GreaterThan,
    LessThan,
    GreaterEquals,
    LessEquals,
    LBS,
    RBS,
    ArrayIndex,
    BinaryAnd,
    BinaryOr,
    And,
    Or,
    Not,
    Negative,
    NotEqual,
    Increment,
    Decrement,
    Optional
}

fn valid_infix_operator(token : &Token) -> bool {
    matches!(token,
        Token::Plus |
        Token::Minus |
        Token::Star |
        Token::Slash |
        Token::Percent |
        Token::LeftShift |
        Token::RightShift |
        Token::EqualsEquals |
        Token::GreaterThan |
        Token::LessThan |
        Token::GreaterThanEquals |
        Token::LessThanEquals |
        Token::And |
        Token::AndAnd |
        Token::Or |
        Token::OrOr
    )
}

fn valid_prefix_operator(token : &Token) -> bool {
    matches!(token, Token::Minus | Token::Not | Token::Increment | Token::Decrement)
}

fn valid_postfix_operator(token : &Token) -> bool {
    matches!(token, Token::OpenBracket | Token::Increment | Token::Decrement)
}

fn create_prefix_operator(entry : TokenEntry) -> Either<impl FnOnce(Expression) -> Expression, TokenEntry> {
    if valid_prefix_operator(&entry.token) {
        Either::Left(move |expr| Expression::new(ExpressionType::Operation(match entry.token  {
            Token::Minus => Operation::Negative(expr),
            Token::Not => Operation::Not(expr),
            Token::Increment => Operation::Increment(expr),
            Token::Decrement => Operation::Decrement(expr),
            _ => unreachable!()
        }), entry.line_number) )
    } else {
        Either::Right(entry)
    }
}

fn create_postfix_operator(entry : TokenEntry) -> Either<Either<impl FnOnce(Expression) -> Expression, impl FnOnce(Expression, Expression) -> Expression>, TokenEntry> {
    if valid_postfix_operator(&entry.token) {
        Either::Left(
            if !matches!(entry.token, Token::OpenBracket) {
                Either::Left(move |expr| Expression::new(ExpressionType::Operation(match entry.token {
                    Token::Increment => Operation::Increment(expr),
                    Token::Decrement => Operation::Decrement(expr),
                    _ => unreachable!()
                }), entry.line_number))
            } else {
                Either::Right(
                    move |index, expr| Expression::new(ExpressionType::Operation(Operation::ArrayIndex(expr, index)), entry.line_number)
                )
            }
        )
    } else {
        Either::Right(entry)
    }
}

fn create_infix_operator<T>(token: TokenEntry) -> Result<impl FnOnce(Expression, Expression) -> Expression, String>  {
    if valid_infix_operator(&token.token) {
        Ok(move |f, s| {
            Expression::new(ExpressionType::Operation(match token.token {
                Token::Plus => Operation::And(f, s),
                Token::Minus => Operation::Sub(f, s),
                Token::Star => Operation::Mul(f, s),
                Token::Slash => Operation::Div(f, s),
                Token::Percent => Operation::Mod(f, s),
                Token::LeftShift => Operation::LBS(f, s),
                Token::RightShift => Operation::RBS(f, s),
                Token::EqualsEquals => Operation::Equals(f, s),
                Token::GreaterThan => Operation::GreaterThan(f, s),
                Token::LessThan => Operation::LessThan(f, s),
                Token::GreaterThanEquals => Operation::GreaterEquals(f, s),
                Token::LessThanEquals => Operation::LessEquals(f, s),
                Token::And => Operation::BinaryAnd(f, s),
                Token::AndAnd => Operation::And(f, s),
                Token::Or => Operation::BinaryOr(f, s),
                Token::OrOr => Operation::Or(f, s),
                _ => unreachable!()
            }), token.line_number)
        })
    } else {
        Err(format!("failed to parse operator '{:?}' on line {}", token.token, token.line_number))
    }
}

#[derive(Debug, Clone)]
pub enum ExpressionType {
    Operation(Operation),
    Field(String),
    Call(String, Vec<Expression>), //method location identifier, parameters
    StringLiteral(String),
    CharLiteral(char),
    BoolLiteral(bool),
    IntLiteral(i64),
    FloatLiteral(f64),
    ListLiteral(Vec<Expression>),
    Empty,
}

#[derive(Debug)]
pub(crate) struct Statement {
    pub(crate) type_ : Box<StatementType>,
    pub(crate) line_number : usize
}

impl Statement {

    pub fn new(type_ : StatementType, line_number : usize) -> Statement {
        Statement { type_: Box::from(type_), line_number }
    }

}

#[derive(Debug)]
pub enum StatementType {
    Assignment(Expression, Expression),
    If(Expression, Vec<Statement>, Option<Statement>), // Condition, body, else-brand
    While(Expression, Vec<Statement>),
    For(String, String, Vec<Statement>),
    Return(Option<Expression>),
    Ignored(Expression),
    Declaration(Expression, Type, Expression)
}

impl Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Operation::Construction(_) => "$",
            Operation::Add(_, _) => "+",
            Operation::Sub(_, _) | Operation::Negative(_) => "-",
            Operation::Mul(_, _) => "*",
            Operation::Div(_, _) => "/",
            Operation::Mod(_, _) => "%",
            Operation::Equals(_, _) => "==",
            Operation::GreaterThan(_, _) => ">",
            Operation::LessThan(_, _) => "<",
            Operation::LBS(_, _) => "<<",
            Operation::RBS(_, _) => ">>",
            Operation::ArrayIndex(_, _) => "[]",
            Operation::BinaryAnd(_, _) => "&",
            Operation::BinaryOr(_, _) => "|",
            Operation::And(_, _)=> "&&",
            Operation::Or(_, _) => "||",
            Operation::Not(_) => "!",
            Operation::GreaterEquals(_, _) => ">=",
            Operation::LessEquals(_, _) => "<",
            Operation::NotEqual(_, _) => "!=",
            Operation::Increment(_) => "++",
            Operation::Decrement(_) => "--",
            Operation::Optional(_) => "?"
        })
    }
}
