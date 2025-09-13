use std::fmt::{Display, Formatter};

pub(crate) struct Function {

    pub(crate) name: String,
    pub(crate) parameters: Vec<Parameter>,
    pub(crate) return_type: Option<Type>,
    pub(crate) statements: Vec<Statement>

}

#[derive(Debug)]
pub(crate) enum Type {
    Int,
    String,
    Float,
    Complex,
    IntList,
    FloatList,
    ComplexList,
    Matrix
}

pub(crate) struct  Parameter {
    pub(crate) name: String,
    pub(crate) typetype: Type
}

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Char(char),
    Bool(bool),
    Int(i64),
    Float(f64),
    Null
}

#[derive(Debug, Clone)]
pub enum ExpressionType {
    Operation(Operation),
    Field(Path),
    Call(Path, Vec<Expression>), //method location identifier, parameters
    Literal(Literal),
    Constructor(Path, Vec<Expression>), //method location identifier, parameters,
    Empty,
}

#[derive(Debug)]
pub struct Statement {
    pub(crate) type_ : Box<StatementType>,
    pub(crate) line_number : usize
}

impl Statement {

    pub fn new(type_ : StatementType, line_number : usize) -> Statement {
        Statement { type_: Box::from(type_), line_number }
    }

}

#[derive(Debug, Clone)]
pub struct Expression {
    pub(crate) type_ : Box<ExpressionType>,
    pub(crate) line_number : usize
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
    Construction,
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

#[derive(Debug)]
pub enum StatementType {
    Assignment(Expression, Expression),
    Fill(Expression, Vec<Expression>), //struct/record identifier, parameters
    If(Expression, Vec<Statement>, Option<Statement>), // Condition, body, else-brand
    While(Expression, Vec<Statement>),
    For(Statement, Expression, Expression, Vec<Statement>),
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